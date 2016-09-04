
use action;
use action::Action;
use confirmation;
use confirmation::Confirmation;
use map::Map;
use player::Player;
use room::Room;
use std::io;
use std::rc::Rc;

pub struct Game<'a> {
	player: Player,
	map: &'a mut Map,
	current_room: Option<Rc<Room>>,
}

impl<'a> Game<'a> {
	pub fn new(player: Player, map: &mut Map) -> Game {
		Game{player: player, map: map, current_room: None}
	}

	pub fn start(&'a mut self) {
		println!("Game Started");

		let mut running = true;
		while running {
			let room:Rc<Room> = match self.current_room {
				Some(ref ref_room) => ref_room.clone(),
				None => {
					self.current_room = Some(self.map.get_start_room());
					self.current_room.clone().unwrap()
				}
			};
			println!("You've entered \"{}\"", room.get_title());
			if !room.is_visited() {
				println!("{}", room.get_first_description());
				room.mark_visited();
			}
			println!("What would you like to do?");

			let mut action = String::new();
			io::stdin().read_line(&mut action).expect("Failed to read line...");
			match action::parse_action(&action) {
				Action::Quit => if self.user_confirms_quit() {running = false;},
				Action::Look(_) => println!("{}", room.get_description()),
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
