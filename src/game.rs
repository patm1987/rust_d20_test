
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

	pub fn start(&mut self) -> String {
		let mut out_str: String = "Game Started".into();
		self.current_room = Some(self.map.get_start_room());
		out_str.push('\n');
		let current_room: Rc<Room> = self.get_current_room();
		out_str.push_str(current_room.get_first_description());
		current_room.mark_visited();

		out_str
	}

	pub fn get_current_room(&self) -> Rc<Room> {
		self.current_room.clone().unwrap()
	}

	pub fn get_player(&self) -> &Player {
		&self.player
	}
}
