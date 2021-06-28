# ndate
This is a CLI program that gives the current Nepali date (Bikram Sambhat date), converts Bikram Sambhat to AD date and vice versa. 

## Description
ndate command can be used to retrieve current Nepali date. 
Options available are -a and -b where -a converts Bikram Sambhat date to AD date and -b converts AD date to bikram sambhat date. 
As an example
ndate
output = 2078-03-13

ndate -b 1985-9-9
output = 2042-05-24

ndate -a 2042-5-24
output = 1985-09-09

Nepali Month can vary from 29 days to 32 days, and they are not fixed so there is not any fixed algorithm to crack the number of days in a nepali month 
for a particular year, therefore starting from 1970 BS to 2090 BS the number of months are hardcoded. Credit to https://github.com/medic/bikram-sambat for 
hardcoded data. 


## Getting Started and Installing
User can download the source code and build it using cargo build and run it using cargo run command if rust is installed in the system, or user can directly 
download the binary provided which will be available soon for various devices and can directly execute the binary. 

### Executing program

* Download the binary 
* Install it in a folder where path is visible, for example /usr/local/bin in mac. 
* Type ndate and it will give you the current nepali date, or use the options provided accordingly. 

## Authors

Contributors names and contact info
Swoven Pokharel 
swovenpokharel@gmail.com
