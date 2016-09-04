extern crate rand;

mod ability;
mod action;
mod confirmation;
mod dice;
mod game;
mod map;
mod player;
mod room;
mod test_world;

use ability::{Ability, AbilityFactory};
use action::Action;
use confirmation::Confirmation;
use game::Game;
use player::PlayerBuilder;
use std::io;

fn main() {
	println!("D20 System Initializing");
	let mut player_builder = PlayerBuilder::new();

	println!("Name your hero");
	let mut name = String::new();
	io::stdin().read_line(&mut name).expect("Failed to read name");

	player_builder.name(name.trim().to_string());

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

	let mut rolls: Vec<Ability> = rolls
		.iter()
		.map(|roll| AbilityFactory::new().roll(*roll).finalize())
		.collect();

	player_builder.strength(select_ability(&mut rolls, "Strength"));
	player_builder.dexterity(select_ability(&mut rolls, "Dexterity"));
	player_builder.constitution(select_ability(&mut rolls, "Constitution"));
	player_builder.intelligence(select_ability(&mut rolls, "Intelligence"));
	player_builder.wisdom(select_ability(&mut rolls, "Wisdom"));
	player_builder.charisma(select_ability(&mut rolls, "Charisma"));

	let player = player_builder.finalize();
	println!("Created player {}", player.get_name());
	println!(
		"STR: {} -> {:+}",
		player.get_strength().get_roll(),
		player.get_strength().get_modifier());
	println!(
		"DEX: {} -> {:+}",
		player.get_dexterity().get_roll(),
		player.get_dexterity().get_modifier());
	println!(
		"CON: {} -> {:+}",
		player.get_constitution().get_roll(),
		player.get_constitution().get_modifier());
	println!(
		"INT: {} -> {:+}",
		player.get_intelligence().get_roll(),
		player.get_intelligence().get_modifier());
	println!(
		"WIS: {} -> {:+}",
		player.get_wisdom().get_roll(),
		player.get_wisdom().get_modifier());
	println!(
		"CHA: {} -> {:+}",
		player.get_charisma().get_roll(),
		player.get_charisma().get_modifier());

	println!("Building game...");
	let mut map = test_world::build_world();
	let mut game = Game::new(player, &mut map);
	println!("{}", game.start());

	let mut running = true;
	while running {
		println!("You're in \"{}\"", game.get_current_room().get_title());
		println!("What would you like to do?");

		let mut action = String::new();
		io::stdin().read_line(&mut action).expect("Failed to read line...");
		match action::parse_action(&action) {
			Action::Quit => if user_confirms_quit(game.get_player().get_name()) {running = false;},
			Action::Look(_) => println!("{}", game.get_current_room().get_description()),
			_ => println!("I didn't understand \"{}\"", action),
		};
	}
}

fn user_confirms_quit(name: &str) -> bool {
	loop {
		println!("Giving up so soon {}?", name);
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

fn select_ability(rolls: &mut Vec<Ability>, name: &str) -> Ability {
	println!("Ability Scores Available:");
	for (index, roll) in rolls.iter().enumerate() {
		println!("[{}] {} -> {}", index, roll.get_roll(), roll.get_modifier());
	}

	loop {
		println!("Select Ability for {}:", name);
		let mut selection = String::new();
		io::stdin().read_line(&mut selection).expect("Failed to read line");
		let selection: i32 = match selection.trim().parse() {
			Ok(num) => num,
			Err(_) => {
				println!("Couldn't parse {}, try again!", selection);
				continue;
			}
		};
		if selection < 0 || selection >= rolls.len() as i32 {
			println!("{} is not a valid index", selection);
			continue;
		}

		return rolls.remove(selection as usize);
	}
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
