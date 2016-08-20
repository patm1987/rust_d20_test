
use map::Map;
use room::Room;

pub fn build_world() -> Map {
	let start_room = Room::new(
		"Foyer",
		"You wake up in an empty foyer. Dust settles around you, your head throbbing. You have no recollection of why you are here",
		"There is a large central staircase. To your left, a bookshelf. To your right, an unlit torch in a wall scone");

	Map::new(start_room)
}
