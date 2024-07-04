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
/// Calculates your love Equinox and prints to terminal
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

