use tiberius::time::time::OffsetDateTime;
use serde::Serialize;
use super::simple_date_time::SimpleDateTime;

/// This date is used as a wrapper for the tiberius "NaiveDateTime" type. This allows us to specify the default date value.
#[derive(Debug, Clone, Serialize)]
pub struct SimpleOffsetDateTime(pub OffsetDateTime);

impl SimpleOffsetDateTime {
	/// Gets the minimum allowed date.
	pub fn get_min_date() -> Self {
		SimpleOffsetDateTime(SimpleDateTime::get_min_date().0.assume_utc())
	}

	/// Gets the maxmium allowed date.
	pub fn get_max_date() -> Self {
		SimpleOffsetDateTime(SimpleDateTime::get_max_date().0.assume_utc())
	}

	/// Determines if the passed in date matches the minimum or maximum date.
	pub fn is_min_or_max_date(date_to_check: &SimpleOffsetDateTime) -> bool {
		SimpleOffsetDateTime::do_dates_match(&SimpleOffsetDateTime::get_min_date(), &date_to_check)
			|| SimpleOffsetDateTime::do_dates_match(&SimpleOffsetDateTime::get_max_date(), &date_to_check)
	}

	/// Determines if too dates match on year, month, and day.
	pub fn do_dates_match(original_date: &SimpleOffsetDateTime, other_date: &SimpleOffsetDateTime) -> bool {
		original_date.0.year() == other_date.0.year() && original_date.0.month() == other_date.0.month() && original_date.0.day() == other_date.0.day()
	}
}

impl Default for SimpleOffsetDateTime {
	fn default() -> Self {
		SimpleOffsetDateTime::get_min_date()
	}
}
