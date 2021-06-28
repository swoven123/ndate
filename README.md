# ndate

This is a CLI program that gives the current Nepali date (Bikram Sambhat date), converts Bikram Sambhat to AD date and vice versa.

## Description

ndate command can be used to retrieve current Nepali date.
Options available are -a and -b where -a converts Bikram Sambhat date to AD date and -b converts AD date to Bikram Sambhat date.

Below are some examples

- ndate

Above command gives the current Nepali date.
output = 2078-03-13

- ndate -b 1985-9-9

Above command converts 1985-9-9 (yyyy-m-d) to Bikram Sambhat date.
output = 2042-05-24

- ndate -a 2042-5-24

Above command converts 2042-5-25 (yyyy-m-d) to AD date.
output = 1985-09-09

Nepali Month can vary from 29 days to 32 days, and they are not fixed so there is not any fixed algorithm to crack the number of days in a nepali month for a particular year, therefore starting from 1970 BS to 2090 BS the number of months are hardcoded. Credit to https://github.com/medic/bikram-sambat for the hardcoded data range (1970 - 2090 BS).

### Note: The conversion only works from the Bikram Sambhat Range of 1970 BS to 2090 BS. If you have more data please feel free to contribute.

## Getting Started and Installing

User can download the source code and build it using cargo build and run it using cargo run command if rust is installed in the system, or user can directly download the binary provided which will be available soon for various devices and can directly execute the binary.

### Executing program

- Download the binary
- Install it in a folder where path is visible, for example /usr/local/bin in mac.
- Type ndate and it will give you the current nepali date, or use the options provided accordingly.

## Authors

Swoven Pokharel: swovenpokharel@gmail.com
