pub struct Room {
	title: String,
	first_description: String,
	description: String,
}

impl Room {
	pub fn new(title: &str, first_description: &str, description: &str) -> Room {
		Room {
			title: title.to_owned(),
			first_description: first_description.to_owned(),
			description: description.to_owned() }
	}

	pub fn get_title(&self) -> &String {
		&self.title
	}

	pub fn get_first_description(&self) -> &String {
		&self.first_description
	}
}
