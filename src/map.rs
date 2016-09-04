
use room::Room;
use std::rc::Rc;

pub struct Map {
	start_room: Rc<Room>,
}

impl Map {
	pub fn new(start_room: Room) -> Map {
		Map{start_room: Rc::new(start_room)}
	}

	pub fn get_start_room(&mut self) -> Rc<Room> {
		self.start_room.clone()
	}
}
