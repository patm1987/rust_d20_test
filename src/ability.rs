
pub struct Ability {
	roll: i32,
	modifier: i32
}

impl Ability {
	pub fn get_roll(&self)->i32 {
		self.roll
	}

	pub fn get_modifier(&self)->i32 {
		self.modifier
	}
}

pub struct AbilityFactory {
	roll: i32,
}

impl AbilityFactory {
	pub fn new() -> AbilityFactory {
		AbilityFactory { roll: 0 }
	}

	pub fn roll(&mut self, roll: i32) -> &mut AbilityFactory {
		self.roll = roll;
		self
	}

	pub fn finalize(&self) -> Ability {
		Ability {
			roll: self.roll,
			modifier: match self.roll {
				2 | 3 => -4,
				4 | 5 => -3,
				6 | 7 => -2,
				8 | 9 => -1,
				10 | 11 => 0,
				12 | 13 => 1,
				14 | 15 => 2,
				16 | 17 => 3,
				18 | 19 => 4,
				_ => if self.roll < 2 { -5 } else { 5 },
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn ability_test_0() {
		let mut factory = AbilityFactory::new();
		factory.roll(0);
		let ability = factory.finalize();
		assert_eq!(ability.get_roll(), 0);
		assert_eq!(ability.get_modifier(), -5);
	}

	#[test]
	fn ability_test_1() {
		let mut factory = AbilityFactory::new();
		factory.roll(1);
		let ability = factory.finalize();
		assert_eq!(ability.get_roll(), 1);
		assert_eq!(ability.get_modifier(), -5);
	}

	#[test]
	fn ability_test_2() {
		let mut factory = AbilityFactory::new();
		factory.roll(2);
		let ability = factory.finalize();
		assert_eq!(ability.get_roll(), 2);
		assert_eq!(ability.get_modifier(), -4);
	}

	#[test]
	fn ability_test_3() {
		let mut factory = AbilityFactory::new();
		factory.roll(3);
		let ability = factory.finalize();
		assert_eq!(ability.get_roll(), 3);
		assert_eq!(ability.get_modifier(), -4);
	}

	#[test]
	fn ability_test_4() {
		let mut factory = AbilityFactory::new();
		factory.roll(4);
		let ability = factory.finalize();
		assert_eq!(ability.get_roll(), 4);
		assert_eq!(ability.get_modifier(), -3);
	}

	#[test]
	fn ability_test_5() {
		let mut factory = AbilityFactory::new();
		factory.roll(5);
		let ability = factory.finalize();
		assert_eq!(ability.get_roll(), 5);
		assert_eq!(ability.get_modifier(), -3);
	}

	#[test]
	fn ability_test_6() {
		let mut factory = AbilityFactory::new();
		factory.roll(6);
		let ability = factory.finalize();
		assert_eq!(ability.get_roll(), 6);
		assert_eq!(ability.get_modifier(), -2);
	}

	#[test]
	fn ability_test_7() {
		let mut factory = AbilityFactory::new();
		factory.roll(7);
		let ability = factory.finalize();
		assert_eq!(ability.get_roll(), 7);
		assert_eq!(ability.get_modifier(), -2);
	}

	#[test]
	fn ability_test_8() {
		let mut factory = AbilityFactory::new();
		factory.roll(8);
		let ability = factory.finalize();
		assert_eq!(ability.get_roll(), 8);
		assert_eq!(ability.get_modifier(), -1);
	}

	#[test]
	fn ability_test_9() {
		let mut factory = AbilityFactory::new();
		factory.roll(9);
		let ability = factory.finalize();
		assert_eq!(ability.get_roll(), 9);
		assert_eq!(ability.get_modifier(), -1);
	}

	#[test]
	fn ability_test_10() {
		let mut factory = AbilityFactory::new();
		factory.roll(10);
		let ability = factory.finalize();
		assert_eq!(ability.get_roll(), 10);
		assert_eq!(ability.get_modifier(), 0);
	}

	#[test]
	fn ability_test_11() {
		let mut factory = AbilityFactory::new();
		factory.roll(11);
		let ability = factory.finalize();
		assert_eq!(ability.get_roll(), 11);
		assert_eq!(ability.get_modifier(), 0);
	}

	#[test]
	fn ability_test_12() {
		let mut factory = AbilityFactory::new();
		factory.roll(12);
		let ability = factory.finalize();
		assert_eq!(ability.get_roll(), 12);
		assert_eq!(ability.get_modifier(), 1);
	}

	#[test]
	fn ability_test_13() {
		let mut factory = AbilityFactory::new();
		factory.roll(13);
		let ability = factory.finalize();
		assert_eq!(ability.get_roll(), 13);
		assert_eq!(ability.get_modifier(), 1);
	}

	#[test]
	fn ability_test_14() {
		let mut factory = AbilityFactory::new();
		factory.roll(14);
		let ability = factory.finalize();
		assert_eq!(ability.get_roll(), 14);
		assert_eq!(ability.get_modifier(), 2);
	}

	#[test]
	fn ability_test_15() {
		let mut factory = AbilityFactory::new();
		factory.roll(15);
		let ability = factory.finalize();
		assert_eq!(ability.get_roll(), 15);
		assert_eq!(ability.get_modifier(), 2);
	}

	#[test]
	fn ability_test_16() {
		let ability = AbilityFactory::new()
			.roll(16)
			.finalize();
		assert_eq!(ability.get_roll(), 16);
		assert_eq!(ability.get_modifier(), 3);
	}

	#[test]
	fn ability_test_17() {
		let ability = AbilityFactory::new()
			.roll(17)
			.finalize();
		assert_eq!(ability.get_roll(), 17);
		assert_eq!(ability.get_modifier(), 3);
	}

	#[test]
	fn ability_test_18() {
		let ability = AbilityFactory::new()
			.roll(18)
			.finalize();
		assert_eq!(ability.get_roll(), 18);
		assert_eq!(ability.get_modifier(), 4);
	}

	#[test]
	fn ability_test_19() {
		let ability = AbilityFactory::new()
			.roll(19)
			.finalize();
		assert_eq!(ability.get_roll(), 19);
		assert_eq!(ability.get_modifier(), 4);
	}

	#[test]
	fn ability_test_20() {
		let ability = AbilityFactory::new()
			.roll(20)
			.finalize();
		assert_eq!(ability.get_roll(), 20);
		assert_eq!(ability.get_modifier(), 5);
	}

	#[test]
	fn ability_test_n1() {
		let ability = AbilityFactory::new()
			.roll(-1)
			.finalize();
		assert_eq!(ability.get_roll(), -1);
		assert_eq!(ability.get_modifier(), -5);
	}
}
