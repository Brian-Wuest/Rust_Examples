use tiberius::time::time::Time;

#[derive(Debug, Clone)]
pub struct SimpleTime(pub Time);

impl SimpleTime {
	/// Gets the minimum allowed time.
	pub fn get_min_time() -> Self {
		SimpleTime(Time::from_hms(0, 0, 0).unwrap())
	}

	/// Gets the maxmium allowed time.
	pub fn get_max_time() -> Self {
		SimpleTime(Time::from_hms(23, 59, 59).unwrap())
	}

	/// Determines if the passed in time matches the minimum or maximum time.
	pub fn is_min_or_max_time(time_to_check: &SimpleTime) -> bool {
		SimpleTime::do_times_match(&SimpleTime::get_min_time(), &time_to_check)
			|| SimpleTime::do_times_match(&SimpleTime::get_max_time(), &time_to_check)
	}

	/// Determines if two times match on hour, minute, and second.
	pub fn do_times_match(original_date: &SimpleTime, other_date: &SimpleTime) -> bool {
		original_date.0.hour() == other_date.0.hour()
			&& original_date.0.minute() == other_date.0.minute()
			&& original_date.0.second() == other_date.0.second()
	}
}

impl Default for SimpleTime {
	fn default() -> Self {
		SimpleTime::get_min_time()
	}
}
