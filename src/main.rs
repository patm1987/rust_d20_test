mod player;

use player::PlayerBuilder;
use std::io;

fn main() {
	println!("D20 System Initializing");
	let mut player_builder = PlayerBuilder::new();

	println!("Name your hero");
	let mut name = String::new();
	io::stdin().read_line(&mut name).expect("Failed to read name");

	player_builder.name(name);
	let player = player_builder.finalize();

	println!("Created player {}", player.get_name());
}