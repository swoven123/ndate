use chrono::{Datelike, NaiveDate, ParseError, TimeZone, Utc};

pub fn parse_date_to_epoch(date: NaiveDate) -> i64 {
  return Utc
    .ymd(date.year(), date.month(), date.day())
    .and_hms(12, 0, 0)
    .timestamp_millis();
}

pub fn parse_date(date: String) -> Result<NaiveDate, ParseError> {
  NaiveDate::parse_from_str(&date[..], "%Y-%m-%d")
}

#[cfg(test)]
mod utils_tests {
  use super::*;

  #[test]
  fn parse_date_test() {
    let formatted_date = NaiveDate::from_ymd(1982, 09, 09);
    let result = parse_date(String::from("1982-09-09"));
    assert_eq!(formatted_date, result.unwrap());
  }

  #[test]
  fn parse_date_test_for_error() {
    let result = parse_date(String::from("1982-09-099"));
    match result {
      Ok(_val) => assert!(false),
      Err(_e) => assert!(true),
    }

    let result = parse_date(String::from("1982-133-09"));
    match result {
      Ok(_val) => assert!(false),
      Err(_e) => assert!(true),
    }
  }

  #[test]
  fn parse_date_to_epoch_test() {
    assert_eq!(
      400420800000,
      parse_date_to_epoch(NaiveDate::from_ymd(1982, 9, 9))
    );
  }
}
