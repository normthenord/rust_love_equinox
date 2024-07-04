use time::{self, Date, Month};
mod libs;
use libs::{le_print, calculate_le};
fn main() {
    let birthday = Date::from_calendar_date(1988, Month::June, 12).unwrap();
    let first_date = Date::from_calendar_date(2006, Month::May, 26).unwrap();

    le_print(first_date, birthday);

    println!("{}", calculate_le(first_date, birthday))
    
}

