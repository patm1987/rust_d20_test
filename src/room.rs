use std::cell::Cell;

pub struct Room {
	title: String,
	first_description: String,
	description: String,
	visited: Cell<bool>,
}

impl Room {
	pub fn new(title: &str, first_description: &str, description: &str) -> Room {
		Room {
			title: title.to_owned(),
			first_description: first_description.to_owned(),
			description: description.to_owned(),
			visited: Cell::new(false) }
	}

	pub fn get_title(&self) -> &String {
		&self.title
	}

	pub fn get_first_description(&self) -> &String {
		&self.first_description
	}

	pub fn get_description(&self) -> &String {
		&self.description
	}

	pub fn mark_visited(&self) {
		self.visited.set(true);
	}

	pub fn is_visited(&self) -> bool {
		self.visited.get()
	}
}
