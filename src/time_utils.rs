use time::Duration;

/// Convert duration to milliseconds.
///
/// For use with `gloo` `Interval`s and `Timeout`s.
///
/// Note this is not appropriate for durations
/// with non-integer number of milliseconds.
pub fn to_millis(period: Duration) -> u32 {
    period.whole_milliseconds() as u32
}

/// Convert duration to string in format `min:sec`.
///
/// # Examples
///
/// ```
/// use time::ext::NumericalDuration;
///
/// assert_eq!("1000:00", to_string(1_000.minutes()))
/// assert_eq!("16:40", to_string(1_000.seconds()))
/// assert_eq!("00:00", to_string(0.seconds()))
/// ```
pub fn to_string(period: Duration) -> String {
    let minutes = period.whole_minutes();
    let seconds = period.whole_seconds() % 60;
    format!("{:02}:{:02}", minutes, seconds)
}
