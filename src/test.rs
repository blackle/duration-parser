#[cfg(test)]
mod tests {
	use parse::parse_duration;
	use error::Error;
	#[test]

	fn five_minutes() {
		let result = parse_duration(&String::from("5m")).unwrap();
		assert!(result == 5 * 60);
	}

	#[test]
	fn combination() {
		let result = parse_duration(&String::from("1y2w5d3h42m10s")).unwrap();
		assert!(
			result
				== 1 * 60 * 60 * 24 * 365 + 2 * 60 * 60 * 24 * 7 + 5 * 60 * 60 * 24 + 3 * 60 * 60
					+ 42 * 60 + 10
		);
	}

	#[test]
	fn overflow() {
		let err = parse_duration(&String::from("1000000000000y")).unwrap_err();
		assert!(err == Error::Overflow);
	}

	#[test]
	fn incorrect_suffix() {
		let err = parse_duration(&String::from("9f")).unwrap_err();
		assert!(err == Error::UnknownSuffix);
	}

	#[test]
	fn unparsable_components() {
		let err = parse_duration(&String::from("1d5ss")).unwrap_err();
		assert!(err == Error::UnparsableExtras);
	}

	#[test]
	fn unparsable_number() {
		let err = parse_duration(&String::from("999999999999999999999999999999999y")).unwrap_err();
		assert!(err == Error::UnparsableNumber);
	}

	#[test]
	fn empty_string() {
		let err = parse_duration(&String::from("")).unwrap_err();
		assert!(err == Error::NoData);
	}

	#[test]
	fn nonsense_string() {
		let err = parse_duration(&String::from("I'm gfucking jormsting?")).unwrap_err();
		assert!(err == Error::NoData);
	}
}
