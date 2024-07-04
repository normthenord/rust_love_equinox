use time::Date;

/// Calculates your love Equinox and returns it as a time::Date
/// You must supply two time::Dates for this function
pub fn calculate_le(first_date: Date, birthday: Date) -> Date {
    let diff = first_date - birthday;
    let equinox = first_date + diff;
    equinox
}
/// Calculates your love Equinox and prints to terminal
/// You must supply two time::Dates for this function
pub fn le_print(first_date: Date, birthday: Date) {
    let diff = first_date - birthday;
    let equinox = first_date + diff;
    println!("{}", equinox);
}
