pub struct Player {
	name: String,
}

pub struct PlayerBuilder {
	name: String,
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

	pub fn finalize(&mut self) -> Player {
		Player {name: self.name.clone()}
	}
}
