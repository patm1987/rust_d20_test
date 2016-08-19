extern crate rand;

mod ability;
mod player;
mod dice;

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

	let roll = dice::roll(3, 6);
	println!("Rolled a {}", roll);

	println!("Created player {}", player.get_name());
}