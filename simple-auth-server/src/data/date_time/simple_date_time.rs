use tiberius::time::time::PrimitiveDateTime;
use serde::Serialize;

use super::{simple_date::SimpleDate, simple_time::SimpleTime};

/// This date is used as a wrapper for the tiberius "NaiveDateTime" type. This allows us to specify the default date value.
#[derive(Debug, Clone, Serialize)]
pub struct SimpleDateTime(pub PrimitiveDateTime);

impl SimpleDateTime {
	/// Gets the minimum allowed date.
	pub fn get_min_date() -> Self {
		SimpleDateTime(PrimitiveDateTime::new(SimpleDate::get_min_date().0, SimpleTime::get_min_time().0))
	}

	/// Gets the maxmium allowed date.
	pub fn get_max_date() -> Self {
		SimpleDateTime(PrimitiveDateTime::new(SimpleDate::get_max_date().0, SimpleTime::get_max_time().0))
	}

	/// Determines if the passed in date matches the minimum or maximum date.
	pub fn is_min_or_max_date(date_to_check: &SimpleDateTime) -> bool {
		SimpleDateTime::do_dates_match(&SimpleDateTime::get_min_date(), &date_to_check)
			|| SimpleDateTime::do_dates_match(&SimpleDateTime::get_max_date(), &date_to_check)
	}

	/// Determines if too dates match on year, month, and day.
	pub fn do_dates_match(original_date: &SimpleDateTime, other_date: &SimpleDateTime) -> bool {
		original_date.0.year() == other_date.0.year()
			&& original_date.0.month() == other_date.0.month()
			&& original_date.0.day() == other_date.0.day()
	}
}

impl Default for SimpleDateTime {
	fn default() -> Self {
		SimpleDateTime::get_min_date()
	}
}
