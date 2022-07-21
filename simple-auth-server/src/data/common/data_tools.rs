use tiberius::{
	numeric::Decimal, time::time::Date, time::time::OffsetDateTime, time::time::PrimitiveDateTime, time::time::Time, Row, Uuid,
};

use crate::data::date_time::{
	simple_date::SimpleDate, simple_date_time::SimpleDateTime, simple_offset_date_time::SimpleOffsetDateTime, simple_time::SimpleTime,
};

pub struct DataTools {}

impl DataTools {
	pub fn get_uuid_and_increment(start_index: &mut usize, row: &Row) -> Uuid {
		let result: Option<Uuid> = row.get(*start_index);
		*start_index += 1;

		match result {
			Some(actual_result) => actual_result,
			None => Uuid::default(),
		}
	}

	pub fn get_uuid_as_option_and_increment(start_index: &mut usize, row: &Row) -> Option<Uuid> {
		let result: Option<Uuid> = row.get(*start_index);
		*start_index += 1;

		match result {
			Some(actual_result) => Some(actual_result),
			None => None,
		}
	}

	pub fn get_string_and_increment(start_index: &mut usize, row: &Row) -> String {
		let result: Option<&str> = row.get(*start_index);
		*start_index += 1;

		match result {
			Some(actual_result) => actual_result.to_string(),
			None => String::default(),
		}
	}

	pub fn get_string_as_option_and_increment(start_index: &mut usize, row: &Row) -> Option<String> {
		let result: Option<&str> = row.get(*start_index);
		*start_index += 1;

		match result {
			Some(actual_result) => Some(actual_result.to_string()),
			None => None,
		}
	}

	pub fn get_bool_and_increment(start_index: &mut usize, row: &Row) -> bool {
		let result: Option<bool> = row.get(*start_index);
		*start_index += 1;

		match result {
			Some(actual_result) => actual_result,
			None => bool::default(),
		}
	}

	pub fn get_i32_and_increment(start_index: &mut usize, row: &Row) -> i32 {
		let result: Option<i32> = DataTools::get_i32_as_option_and_increment(start_index, row);

		match result {
			Some(actual_result) => actual_result,
			None => i32::default(),
		}
	}

	pub fn get_i32_as_option_and_increment(start_index: &mut usize, row: &Row) -> Option<i32> {
		let result: Option<i32> = row.get(*start_index);
		*start_index += 1;

		result
	}

	pub fn get_i16_and_increment(start_index: &mut usize, row: &Row) -> i16 {
		let result: Option<i16> = DataTools::get_i16_as_option_and_increment(start_index, row);

		match result {
			Some(actual_result) => actual_result,
			None => i16::default(),
		}
	}

	pub fn get_i16_as_option_and_increment(start_index: &mut usize, row: &Row) -> Option<i16> {
		let result: Option<i16> = row.get(*start_index);
		*start_index += 1;

		result
	}

	pub fn get_u8_as_option_and_increment(start_index: &mut usize, row: &Row) -> Option<u8> {
		let result: Option<u8> = row.get(*start_index);
		*start_index += 1;

		result
	}

	pub fn get_varbinary_as_option_and_increment(start_index: &mut usize, row: &Row) -> Option<Vec<u8>> {
		let result: Option<&[u8]> = row.get(*start_index);
		*start_index += 1;

		match result {
			Some(array) => Some(array.to_vec()),
			None => None,
		}
	}

	pub fn get_decimal_as_option_and_increment(start_index: &mut usize, row: &Row) -> Option<Decimal> {
		let result: Option<Decimal> = row.get(*start_index);
		*start_index += 1;

		result
	}

	pub fn get_simple_date_and_increment(start_index: &mut usize, row: &Row) -> SimpleDate {
		let result: Option<SimpleDate> = DataTools::get_simple_date_as_option_and_increment(start_index, row);

		match result {
			Some(actual_result) => actual_result,
			None => SimpleDate::default(),
		}
	}

	pub fn get_simple_date_as_option_and_increment(start_index: &mut usize, row: &Row) -> Option<SimpleDate> {
		let result: Option<Date> = row.get(*start_index);
		*start_index += 1;

		match result {
			Some(actual_result) => Some(SimpleDate(actual_result)),
			None => None,
		}
	}

	pub fn get_simple_time_as_option_and_increment(start_index: &mut usize, row: &Row) -> Option<SimpleTime> {
		let result: Option<Time> = row.get(*start_index);
		*start_index += 1;

		match result {
			Some(actual_result) => Some(SimpleTime(actual_result)),
			None => None,
		}
	}

	pub fn get_simple_date_time_and_increment(start_index: &mut usize, row: &Row) -> SimpleDateTime {
		let result: Option<PrimitiveDateTime> = row.get(*start_index);
		*start_index += 1;

		match result {
			Some(actual_result) => SimpleDateTime(actual_result),
			None => SimpleDateTime::get_min_date(),
		}
	}

	pub fn get_simple_date_time_as_option_and_increment(start_index: &mut usize, row: &Row) -> Option<SimpleDateTime> {
		let result: Option<PrimitiveDateTime> = row.get(*start_index);

		*start_index += 1;

		match result {
			Some(actual_result) => Some(SimpleDateTime(actual_result)),
			None => None,
		}
	}

	pub fn get_simple_date_time_offset_and_increment(start_index: &mut usize, row: &Row) -> SimpleOffsetDateTime {
		let result: Option<OffsetDateTime> = row.get(*start_index);

		*start_index += 1;

		// The below would work but the row is behind a reference and "into_iter" moves the reference which is not allowed
		// DateTimeOffSets are not really supported so there is no good way to get this information out of the database.
		/* for item in cloned_row.into_iter() {
				match item {
						ColumnData::DateTimeOffset(ref val) => match val {
								Some(dto) => {
										let date =
												NaiveDate::from_num_days_from_ce(dto.datetime2().date().days() as i32);
										let ns = dto.datetime2().time().increments() as i64
												* 10i64.pow(9 - dto.datetime2().time().scale() as u32);
										let time = NaiveTime::from_hms(0, 0, 0) + chrono::Duration::nanoseconds(ns)
												- chrono::Duration::minutes(dto.offset() as i64);
										let naive = NaiveDateTime::new(date, time);
										return SimpleDateTime(naive);
								}
								None => {}
						},
						_ => {}
				};
		} */

		// Datetime offset does not have a good option
		match result {
			Some(actual_result) => SimpleOffsetDateTime(actual_result),
			None => SimpleOffsetDateTime::get_min_date(),
		}
	}

	pub fn get_simple_date_time_offset_as_option_and_increment(start_index: &mut usize, row: &Row) -> Option<SimpleOffsetDateTime> {
		let result: Option<OffsetDateTime> = row.get(*start_index);

		*start_index += 1;

		match result {
			Some(actual_result) => Some(SimpleOffsetDateTime(actual_result)),
			None => None,
		}
	}
}
