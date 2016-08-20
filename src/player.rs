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
		return &self.name;
	}
}

impl PlayerBuilder {
	pub fn new() -> PlayerBuilder {
		PlayerBuilder { name: "None".to_string() }
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

	pub fn wisdom(&mut sellf, wisdom: Ability) -> &mut PlayerBuilder {
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
			strength: self.strength,
			dexterity: self.dexterity,
			constitution: self.constitution,
			intelligence: self.intelligence,
			wisdom: self.wisdom,
			charisma: self.charisma
		}
	}
}
