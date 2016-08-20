pub enum Action {
	Quit,
	Look,
	Unknown
}

pub fn parse_action(action_string: &str) -> Action {
	let lower_string = action_string.to_lowercase();
	if lower_string.starts_with("look") {return Action::Look;}
	if lower_string.starts_with("quit") {return Action::Quit;}
	Action::Unknown
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_parses_look() {
		match parse_action("look") {
			Action::Look => (),
			_ => assert!(false)
		}
	}

	#[test]
	fn test_handles_nonsense() {
		match parse_action("alsdl;flahsdhjkl;jkl;hhjl;") {
			Action::Unknown => (),
			_ => assert!(false)
		}
	}

	#[test]
	fn test_look_handles_mixed_case() {
		match parse_action("LoOk") {
			Action::Look => (),
			_ => assert!(false)
		}
	}

	#[test]
	fn test_quit() {
		match parse_action("quit") {
			Action::Quit => (),
			_ => assert!(false)
		}
	}
}
