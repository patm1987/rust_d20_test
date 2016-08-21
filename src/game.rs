
use action;
use action::Action;
use confirmation;
use confirmation::Confirmation;
use map::Map;
use player::Player;
use room::Room;
use std::io;

pub struct Game<'a> {
	player: Player,
	map: &'a Map,
	current_room: &'a Room
}

impl<'a> Game<'a> {
	pub fn new(player: Player, map: &'a Map) -> Game<'a> {
		Game{player: player, map: map, current_room: map.get_start_room()}
	}

	pub fn start(&mut self) {
		println!("Game Started");

		let mut running = true;
		while running {
			println!("You've entered \"{}\"", self.current_room.get_title());
			println!("{}", self.current_room.get_first_description());
			println!("What would you like to do?");

			let mut action = String::new();
			io::stdin().read_line(&mut action).expect("Failed to read line...");
			match action::parse_action(&action) {
				Action::Quit => if self.user_confirms_quit() {running = false;},
				Action::Look => println!("{}", self.current_room.get_description()),
				_ => println!("I didn't understand \"{}\"", action),
			};
		}
	}

	fn user_confirms_quit(&self) -> bool {
		loop {
			println!("Giving up so soon {}?", self.player.get_name());
			let mut confirmation = String::new();
			io::stdin().read_line(&mut confirmation).expect("Failed to read line...");
			match confirmation::parse_confirmation(&confirmation) {
				Confirmation::Yes => return true,
				Confirmation::No => return false,
				_ => {
					println!("What was that?");
					continue
				}
			}
		}
	}
}
