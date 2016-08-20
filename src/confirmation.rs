
pub fn parse_confirmation(confirmation: &str) -> bool {
	let confirmation = confirmation.to_lowercase();
	if confirmation.starts_with("y") {return true;}
	return false;
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn parses_y () {
		assert!(parse_confirmation("y"));
	}

	#[test]
	fn parses_n() {
		assert!(!parse_confirmation("n"));
	}

	#[test]
	fn parses_capital_y() {
		assert!(parse_confirmation("Y"));
	}

	#[test]
	fn parses_yes() {
		assert!(parse_confirmation("Yes"));
	}
}
