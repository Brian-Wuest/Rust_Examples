use tiberius::time::time::Date;
use time::Month;

/// This date is used as a wrapper for the tiberius "NaiveDate" type. This allows us to specify the default date value.
#[derive(Debug, Clone)]
pub struct SimpleDate(pub Date);

impl SimpleDate {
	/// Gets the minimum allowed date.
	pub fn get_min_date() -> Self {
		SimpleDate(Date::from_calendar_date(1900, Month::January, 1).unwrap())
	}

	/// Gets the maxmium allowed date.
	pub fn get_max_date() -> Self {
		SimpleDate(Date::from_calendar_date(9999, Month::December, 31).unwrap())
	}

	/// Determines if the passed in date matches the minimum or maximum date.
	pub fn is_min_or_max_date(date_to_check: &SimpleDate) -> bool {
		SimpleDate::do_dates_match(&SimpleDate::get_min_date(), &date_to_check)
			|| SimpleDate::do_dates_match(&SimpleDate::get_max_date(), &date_to_check)
	}

	/// Determines if too dates match on year, month, and day.
	pub fn do_dates_match(original_date: &SimpleDate, other_date: &SimpleDate) -> bool {
		original_date.0.year() == other_date.0.year()
			&& original_date.0.month() == other_date.0.month()
			&& original_date.0.day() == other_date.0.day()
	}
}

impl Default for SimpleDate {
	fn default() -> Self {
		SimpleDate::get_min_date()
	}
}
