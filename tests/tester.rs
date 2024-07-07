#[cfg(test)]

mod tests {
    use time::Date;

    #[test]
    fn it_works(){
        let first_date = Date::from_calendar_date(2006, time::Month::May, 26).unwrap();
        let birthday = Date::from_calendar_date(1988, time::Month::June, 12).unwrap();
        assert_eq!(love_equinox::calculate_le(first_date, birthday).unwrap(), Date::from_calendar_date(2024, time::Month::May, 8).unwrap());
    }
    #[test]
    fn it_works2(){
        let first_date = Date::from_calendar_date(2006, time::Month::May, 26).unwrap();
        let birthday = Date::from_calendar_date(1988, time::Month::August, 18).unwrap();
        assert_eq!(love_equinox::calculate_le(first_date, birthday).unwrap(), Date::from_calendar_date(2024, time::Month::March, 2).unwrap());
    }
    #[test]
    fn bad_info(){
        let first_date = Date::from_calendar_date(2006, time::Month::May, 26).unwrap();
        let birthday = Date::from_calendar_date(1988, time::Month::June, 12).unwrap();
        assert_eq!(love_equinox::calculate_le(birthday, first_date), Err("You weren't born when the relationship started!".to_owned()));    }

    #[test]
    fn join_test_print(){
        let first_date = Date::from_calendar_date(2006, time::Month::May, 26).unwrap();
        let birthday_one = Date::from_calendar_date(1988, time::Month::June, 12).unwrap();
        let birthday_two = Date::from_calendar_date(1988, time::Month::August, 18).unwrap();
        love_equinox::print_joint_le(first_date, birthday_one, birthday_two);
    }

    #[test]
    fn join_test(){
        let first_date = Date::from_calendar_date(2006, time::Month::May, 26).unwrap();
        let birthday_one = Date::from_calendar_date(1988, time::Month::June, 12).unwrap();
        let birthday_two = Date::from_calendar_date(1988, time::Month::August, 18).unwrap();
        assert_eq!(love_equinox::calculate_joint_le(first_date, birthday_one, birthday_two).unwrap(), Date::from_calendar_date(2024, time::Month::April, 4).unwrap());
    }

}