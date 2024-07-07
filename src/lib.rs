use time::Date;


/// Calculates your love Equinox and returns it as a time::Date
/// You must supply two time::Dates for this function
pub fn calculate_le(first_date: Date, birthday: Date) -> Result<Date,String> {
    if first_date < birthday{
        return Err("You weren't born when the relationship started!".to_owned())
    }
    let diff = first_date - birthday;
    let equinox = first_date + diff;
    Ok(equinox)
}
/// Calculates your love Equinox and printlns to terminal
/// You must supply two time::Dates for this function
pub fn le_print(first_date: Date, birthday: Date) {
    if first_date < birthday{
        println!("You weren't born when the relationship started!");
        return
    }
    let diff = first_date - birthday;
    let equinox = first_date + diff;
    println!("{}", equinox);
}

/// Calculates you and your partner's join Love Equinox and
/// returns it as a time::Date
/// You must supply three time::Dates for this function
pub fn calculate_joint_le(first_date: Date, birthday_one: Date, birthday_two: Date) -> Result<Date, String>{
    if first_date < birthday_one || first_date < birthday_two{
        return Err("Not everyone was born when the relationship started!".to_owned())
    }
    let diff_one = first_date - birthday_one;
    let diff_two = first_date - birthday_two;
    let diff = (diff_one + diff_two)/2;
    let equinox = first_date + diff;
    Ok(equinox)
}
/// Calculates you and your partner's join Love Equinox and
/// printlns it to your terminal
/// You must supply three time::Dates for this function
pub fn print_joint_le(first_date: Date, birthday_one: Date, birthday_two: Date){
    if first_date < birthday_one || first_date < birthday_two{
        println!("Not everyone was born when the relationship started!");
        return
    }
    let diff_one = first_date - birthday_one;
    let diff_two = first_date - birthday_two;
    let diff = (diff_one + diff_two)/2;
    let equinox = first_date + diff;

    println!("{:?}", equinox)
}