
pub enum Confirmation {
	Yes,
	No,
	Unknown
}

pub fn parse_confirmation(confirmation: &str) -> Confirmation {
	let confirmation = confirmation.trim().to_lowercase();
	if confirmation == "y" || confirmation == "yes" {return Confirmation::Yes;}
	else if confirmation == "n" || confirmation == "no" {return Confirmation::No;}
	Confirmation::Unknown
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn parses_y () {
		assert!(match parse_confirmation("y") {
			Confirmation::Yes => true,
			_ => false,
		});
	}

	#[test]
	fn parses_n() {
		assert!(match parse_confirmation("n") {
			Confirmation::No => true,
			_ => false
		});
	}

	#[test]
	fn parses_capital_y() {
		assert!(match parse_confirmation("Y") {
			Confirmation::Yes => true,
			_ => false
		});
	}

	#[test]
	fn parses_yes() {
		assert!(match parse_confirmation("Yes") {
			Confirmation::Yes => true,
			_ => false
		});
	}

	#[test]
	fn parses_nonsense() {
		assert!(match parse_confirmation("xkcd") {
			Confirmation::Unknown => true,
			_ => false
		});
	}

	#[test]
	fn ignores_y_on_nonsense() {
		assert!(match parse_confirmation("yggdrassil") {
			Confirmation::Unknown => true,
			_ => false
		});
	}

	#[test]
	fn ignores_n_on_nonsense() {
		assert!(match parse_confirmation("nvidia") {
			Confirmation::Unknown => true,
			_ => false
		});
	}

	#[test]
	fn parses_no() {
		assert!(match parse_confirmation("no") {
			Confirmation::No => true,
			_ => false
		});
	}
}
