
use player::Player;
use map::Map;

pub struct Game {
	player: Player,
	map: Map,
}

impl Game {
	pub fn new(player: Player, map: Map) -> Game {
		Game{player: player, map: map}
	}

	pub fn start(&mut self) -> () {
		println!("Game Started");
	}
}
