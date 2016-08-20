pub enum Action {
	Look,
	Unknown
}

pub fn parse_action(action_string: &str) -> Action {
	if action_string.to_lowercase().starts_with("look") {return Action::Look;}
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
}
