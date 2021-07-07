use chrono::{Datelike, NaiveDate, TimeZone, Utc};
use std::convert::TryInto;

mod constants;
mod utils;

use constants::{
  BS_EPOCH_TIMESTAMP, BS_YEAR_ZERO, FORMAT_CORRECTLY_ERROR_MESSAGE, INTERNAL_PROGRAM_ERROR_MESSAGE,
  MONTH_LENGTHS_ENCODED, MS_PER_DAY,
};

/// Takes in the input as Config struct and displays converted date
/// if the operations are successful, else returns error.
///
/// # Examples
/// ```
///
/// use ndate::Config;
///
/// let args = vec![String::from("ndate"),String::from("-b"),String::from("1985-09-09")];
/// let working_config = Config::new(&args).unwrap();
/// ndate::execute(working_config).unwrap_or_else(|_err| assert!(false));
///
/// ```
pub fn execute<'a>(config: Config) -> Result<(), &'a str> {
  let result = if config.bs {
    ad_to_bs(config.date)
  } else {
    bs_to_ad(config.date)
  };

  match result {
    Ok(result) => println!("{}", result),
    Err(e) => return Err(e),
  }
  Ok(())
}

fn bs_to_ad<'a>(date: NaiveDate) -> Result<String, &'a str> {
  let mut year: i32 = date.year();
  let mut month: u32 = date.month();
  let day: u32 = date.day();

  let mut timestamp_with_day: i64 = BS_EPOCH_TIMESTAMP + (i64::from(MS_PER_DAY * day));
  month -= 1;

  while year >= BS_YEAR_ZERO {
    while month > 0 {
      let days_in_month_millis = calculate_days_in_month(year, month) * MS_PER_DAY;
      timestamp_with_day += i64::from(days_in_month_millis);
      month -= 1;
    }
    month = 12;
    year -= 1;
  }
  let date_with_time_str: String = Utc.timestamp_millis(timestamp_with_day).to_rfc3339();
  let date_with_time_vec: Vec<&str> = date_with_time_str.split("T").collect();

  if let Some(value) = date_with_time_vec.get(0) {
    return Ok(value.to_string());
  }
  Err(INTERNAL_PROGRAM_ERROR_MESSAGE)
}

fn ad_to_bs<'a>(date: NaiveDate) -> Result<String, &'a str> {
  let mut year = BS_YEAR_ZERO;
  let utc_date_timestamp = utils::parse_date_to_epoch(date);

  //This is the number of days after epoch time which is 1970-01-01 BS or 1913-4-13 AD
  let millis_difference_from_epoch = utc_date_timestamp - BS_EPOCH_TIMESTAMP;
  let days_in_float: f64 = millis_difference_from_epoch as f64 / f64::from(MS_PER_DAY);
  let mut days: u32 = days_in_float.floor() as u32 + 1;
  while days > 0 {
    for i in 1..13 {
      let days_in_month = calculate_days_in_month(year, i);
      if days <= days_in_month {
        let bs_date_string = format!(
          "{}-{}-{}",
          year,
          format!("{:0>2}", i),
          format!("{:0>2}", days)
        );
        return Ok(bs_date_string);
      }
      days -= days_in_month;
    }
    year += 1;
  }
  Err(INTERNAL_PROGRAM_ERROR_MESSAGE)
}

fn calculate_days_in_month(year: i32, month: u32) -> u32 {
  let shifting_val = (month - 1) << 1;
  //A nepali month can have a smallest val as 29
  (29 + ((MONTH_LENGTHS_ENCODED[(year - BS_YEAR_ZERO) as usize] >> shifting_val) & 3))
    .try_into()
    .unwrap()
}

/// Config struct is used to wrap the arguments passed from user to one common place
///
/// # Examples
/// ```
/// let args = vec![String::from("ndate"),String::from("-b"),String::from("1985-09-09")];
//let test_config = Config::new(&args).unwrap();
/// ```
pub struct Config {
  bs: bool,
  date: NaiveDate,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, &str> {
    if args.len() == 1 {
      let current_date = Utc::now();
      return Ok(Config {
        bs: true,
        date: NaiveDate::from_ymd(
          current_date.year(),
          current_date.month(),
          current_date.day(),
        ),
      });
    }
    if args.len() == 2 || args.len() > 3 {
      return Err("Incorrect number of arguments passed");
    }
    let bs = if args[1].eq("-b") {
      true
    } else if args[1].eq("-a") {
      false
    } else {
      return Err("Incorrect argument passed");
    };
    let date = utils::parse_date(String::from(args[2].clone()));
    match date {
      Ok(date) => return Ok(Config { bs, date }),
      Err(_parse_error) => return Err(FORMAT_CORRECTLY_ERROR_MESSAGE),
    };
  }
}

#[cfg(test)]
mod calculations_tests {
  use super::*;

  #[test]
  fn test_calculate_days_in_month() {
    assert_eq!(30, calculate_days_in_month(1985, 9));
  }

  #[test]
  fn test_bs_to_ad() {
    assert_eq!(
      "2042-05-24",
      ad_to_bs(NaiveDate::from_ymd(1985, 09, 09)).unwrap()
    );
  }

  #[test]
  fn test_ad_to_bs() {
    assert_eq!(
      "2021-06-27",
      bs_to_ad(NaiveDate::from_ymd(2078, 03, 13)).unwrap()
    );
  }

  #[test]
  fn test_execute_test_case_1() {
    let args = vec![
      String::from("ndate"),
      String::from("-b"),
      String::from("1985-09-09"),
    ];
    let working_config = Config::new(&args).unwrap();
    execute(working_config).unwrap_or_else(|_err| {
      assert!(false);
    });
  }

  #[test]
  #[should_panic]
  fn test_execute_test_case_2() {
    let args = vec![
      String::from("ndate"),
      String::from("-b"),
      String::from("1985-09-009"),
    ];
    let working_config = Config::new(&args).unwrap();
    execute(working_config).unwrap_or_else(|_err| {});
  }
}
