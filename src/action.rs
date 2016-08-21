pub enum Direction {
	North,
	South,
	East,
	West,
	Unknown
}

pub enum Action {
	Quit,
	Look (Direction),
	Unknown
}

pub fn parse_action(action_string: &str) -> Action {
	let split_string: Vec<String> = action_string
		.to_lowercase()
		.trim()
		.split(" ")
		.map(|str| str.to_string())
		.collect();

	match split_string[0].as_ref() {
		"look" => {
			// there is no second string to match, just go to unknown
			if split_string.len() < 2 {Action::Look(Direction::Unknown)}
			else {
				Action::Look(match split_string[1].as_ref() {
					"north" => Direction::North,
					"south" => Direction::South,
					"east" => Direction::East,
					"west" => Direction::West,
					_ => Direction::Unknown
				})
			}	
		},
		"quit" => Action::Quit,
		_ => Action::Unknown
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_parses_look() {
		assert!(match parse_action("look") {
			Action::Look(Direction::Unknown) => true,
			_ => false
		});
	}

	#[test]
	fn test_handles_nonsense() {
		assert!(match parse_action("alsdl;flahsdhjkl;jkl;hhjl;") {
			Action::Unknown => true,
			_ => false
		});
	}

	#[test]
	fn test_look_handles_mixed_case() {
		assert!(match parse_action("LoOk") {
			Action::Look(Direction::Unknown) => true,
			_ => false
		});
	}

	#[test]
	fn test_look_north() {
		assert!(match parse_action("look north") {
			Action::Look(Direction::North) => true,
			_ => false
		});
	}

	#[test]
	fn test_look_south() {
		assert!(match parse_action("look south") {
			Action::Look(Direction::South) => true,
			_ => false
		});
	}

	#[test]
	fn test_look_east() {
		assert!(match parse_action("look east") {
			Action::Look(Direction::East) => true,
			_ => false
		});
	}

	#[test]
	fn test_look_west() {
		assert!(match parse_action("look west") {
			Action::Look(Direction::West) => true,
			_ => false
		});
	}

	#[test]
	fn test_quit() {
		assert!(match parse_action("quit") {
			Action::Quit => true,
			_ => false
		});
	}

	// todo: test empty string to parse
}
