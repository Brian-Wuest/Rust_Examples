use unicode_segmentation::UnicodeSegmentation;

pub struct StringServices {}

impl StringServices {
	/// Slices a string using graphemes from the given start for the given length.
	pub fn sub_string(value: String, start: usize, len: usize) -> String {
		let mut return_value = "".to_string();
		let end = start + len;

		for (i, el) in value.graphemes(true).enumerate() {
			if i >= start && i <= end {
				return_value = return_value + el;
			}
		}

		return_value
	}

	/// Slices a string using graphemes from the given start for the given length.
	pub fn sub_string_ref(value: &String, start: usize, len: usize) -> String {
		let mut return_value = "".to_string();
		let end = start + len;

		for (i, el) in value.graphemes(true).enumerate() {
			if i >= start && i <= end {
				return_value = return_value + el;
			}
		}

		return_value
	}

	pub fn index_of(value: &String, characters: &str) -> i32 {
		let mut return_value: i32 = -1;

		for (i, el) in value.graphemes(true).enumerate() {
			if el.eq_ignore_ascii_case(characters) {
				return_value = i as i32;
				break;
			}
		}

		return_value
	}

	pub fn ends_with(value: &String, characters: &str) -> bool {
		let start_index = value.len() - characters.len();

		let sub_string = StringServices::sub_string_ref(value, start_index, characters.len());

		sub_string == characters
	}

	pub fn starts_with(value: &String, characters: &str) -> bool {
		let sub_string = StringServices::sub_string_ref(value, 0, characters.len() - 1);

		sub_string == characters
	}
}
