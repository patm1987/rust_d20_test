
use map::Map;
use player::Player;
use room::Room;

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
		println!("You've entered \"{}\"", self.current_room.get_title());
		println!("{}", self.current_room.get_first_description());
	}
}
