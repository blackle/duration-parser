use regex::Regex;
use error::Error;

const SEC_IN_MIN: u64 = 60;
const SEC_IN_HR: u64 = SEC_IN_MIN * 60;
const SEC_IN_DAY: u64 = SEC_IN_HR * 24;
const SEC_IN_WK: u64 = SEC_IN_DAY * 7;
const SEC_IN_YR: u64 = SEC_IN_DAY * 365;

fn multiplier_from_unit(units: &str) -> Result<u64, Error> {
	match units {
		"s" => Ok(1),
		"m" => Ok(SEC_IN_MIN),
		"h" => Ok(SEC_IN_HR),
		"d" => Ok(SEC_IN_DAY),
		"w" => Ok(SEC_IN_WK),
		"y" => Ok(SEC_IN_YR),
		_ => Err(Error::UnknownSuffix),
	}
}

pub fn parse_duration(string: &str) -> Result<u64, Error> {
	lazy_static! {
		static ref TIME_RE: Regex = Regex::new(r"(?P<value>\d+)(?P<units>[a-z])").unwrap();
	}

	let caps_iter = TIME_RE.captures_iter(string);

	let mut expected_length = 0;
	let mut total_duration_in_ms: u64 = 0;
	for caps in caps_iter {
		let value_str = &caps["value"];
		let units = &caps["units"];

		expected_length += value_str.len() + units.len();

		let value: u64 = match value_str.parse() {
			Ok(val) => val,
			Err(_) => return Err(Error::UnparsableNumber),
		};

		let duration_in_ms = match value.checked_mul(multiplier_from_unit(units)?) {
			Some(val) => val,
			None => return Err(Error::Overflow),
		};

		total_duration_in_ms = match total_duration_in_ms.checked_add(duration_in_ms) {
			Some(val) => val,
			None => return Err(Error::Overflow),
		};
	}

	if expected_length == 0 {
		return Err(Error::NoData);
	}

	if expected_length != string.len() {
		return Err(Error::UnparsableExtras);
	}

	Ok(total_duration_in_ms)
}
