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

	let mut rolls;
	loop {
		rolls = vec![
			dice::roll(3, 6),
			dice::roll(3, 6),
			dice::roll(3, 6),
			dice::roll(3, 6),
			dice::roll(3, 6),
			dice::roll(3, 6),
		];
		if rolls_valid(&rolls) {
			break;
		}
	}

	println!("Rolled {} die with results:", rolls.len());
	for roll in rolls {
		println!("\t{}", roll);
	}

	println!("Generating:\n\
		Strength\n\
		Dexterity\n\
		Constitution\n\
		Intelligence\n\
		Wisdom\n\
		Charisma");

	println!("Created player {}", player.get_name());
}

fn rolls_valid(rolls: &Vec<i32>) -> bool {
	if rolls.len() != 6 {
		return false;
	}

	let mut sum: i32 = 0;
	let mut max: i32 = i32::min_value();
	for roll in rolls {
		sum += *roll;
		if *roll > max {
			max = *roll;
		}
	}
	sum > 0 && max > 1
}
