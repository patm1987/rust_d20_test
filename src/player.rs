use ability::{Ability, AbilityFactory};

#[derive(Clone)]
pub struct Player {
	name: String,
	strength: Ability,
	dexterity: Ability,
	constitution: Ability,
	intelligence: Ability,
	wisdom: Ability,
	charisma: Ability
}

pub struct PlayerBuilder {
	name: String,
	strength: Ability,
	dexterity: Ability,
	constitution: Ability,
	intelligence: Ability,
	wisdom: Ability,
	charisma: Ability
}

impl Player {
	pub fn get_name(&self) -> &String {
		&self.name
	}

	pub fn get_strength(&self) -> &Ability {
		&self.strength
	}

	pub fn get_dexterity(&self) -> &Ability {
		&self.dexterity
	}

	pub fn get_constitution(&self) -> &Ability {
		&self.constitution
	}

	pub fn get_intelligence(&self) -> &Ability {
		&self.intelligence
	}

	pub fn get_wisdom(&self) -> &Ability {
		&self.wisdom
	}

	pub fn get_charisma(&self) -> &Ability {
		&self.charisma
	}
}

impl PlayerBuilder {
	pub fn new() -> PlayerBuilder {
		PlayerBuilder {
			name: "None".to_string(),
			strength: AbilityFactory::new().finalize(),
			dexterity: AbilityFactory::new().finalize(),
			constitution: AbilityFactory::new().finalize(),
			intelligence: AbilityFactory::new().finalize(),
			wisdom: AbilityFactory::new().finalize(),
			charisma: AbilityFactory::new().finalize()
		}
	}

	pub fn name(&mut self, name: String) -> &mut PlayerBuilder {
		self.name = name;
		self
	}

	pub fn strength(&mut self, strength: Ability) -> &mut PlayerBuilder {
		self.strength = strength;
		self
	}

	pub fn dexterity(&mut self, dexterity: Ability) -> &mut PlayerBuilder {
		self.dexterity = dexterity;
		self
	}

	pub fn constitution(&mut self, constitution: Ability) -> &mut PlayerBuilder {
		self.constitution = constitution;
		self
	}

	pub fn intelligence(&mut self, intelligence: Ability) -> &mut PlayerBuilder {
		self.intelligence = intelligence;
		self
	}

	pub fn wisdom(&mut self, wisdom: Ability) -> &mut PlayerBuilder {
		self.wisdom = wisdom;
		self
	}

	pub fn charisma(&mut self, charisma: Ability) -> &mut PlayerBuilder {
		self.charisma = charisma;
		self
	}

	pub fn finalize(&mut self) -> Player {
		Player {
			name: self.name.clone(),
			strength: self.strength.clone(),
			dexterity: self.dexterity.clone(),
			constitution: self.constitution.clone(),
			intelligence: self.intelligence.clone(),
			wisdom: self.wisdom.clone(),
			charisma: self.charisma.clone()
		}
	}
}
