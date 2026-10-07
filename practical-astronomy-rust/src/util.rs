/// Determine if year is a leap year.
///
/// ## Arguments
/// year
///
/// ## Returns
/// true or false
pub fn is_leap_year(input_year: u32) -> bool {
    let year = input_year as f64;

    if year % 4.0 == 0.0 {
        if year % 100.0 == 0.0 {
            return ternary_assign(year % 400.0 == 0.0, true, false);
        } else {
            return true;
        }
    } else {
        return false;
    }
}

/// Round an f64 primitive to the specified number of decimal places.
pub fn round_f64(input_value: f64, places: usize) -> f64 {
    return format!("{:.width$}", input_value, width = places)
        .parse::<f64>()
        .unwrap();
}

/// Convert a Universal Time hour to Local Time
pub fn get_local_hour_from_ut(
    input_hour: f64,
    is_daylight_saving: bool,
    zone_correction_hours: i32,
) -> f64 {
    let adjustment_value = ternary_assign(
        is_daylight_saving,
        (zone_correction_hours as f64) - 1.0,
        zone_correction_hours as f64,
    );

    let local_hour: f64 = input_hour - adjustment_value;

    return local_hour;
}

/// Return one of two possible values, dependent upon the value of is_true_state
pub fn ternary_assign<T>(is_true_state: bool, true_state_value: T, false_state_value: T) -> T {
    if is_true_state {
        return true_state_value;
    }

    return false_state_value;
}

/// Given a boolean value, return a corresponding int (1 or 0)
pub fn bool_to_int(input_bool: bool) -> i32 {
    return ternary_assign(input_bool, 1, 0);
}
