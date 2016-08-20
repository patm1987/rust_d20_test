
use room::Room;

pub struct Map {
	start_room: Room,
}

impl Map {
	pub fn new(start_room: Room) -> Map {
		Map{start_room: start_room}
	}

	pub fn get_start_room<'a>(&'a self) -> &'a Room {
		&self.start_room
	}
}
