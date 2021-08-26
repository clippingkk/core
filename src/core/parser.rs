use std::io;

mod core {
	pub struct Clipping {
		title: String,
		content: String,
		pageAt: String,
		bookId: String,
		createdAt: String,
	}

	pub fn Parse(src: String) -> Result<Vec<Clipping>, io::Error> {
		let mut result: Vec<Clipping> = Vec::new();
		return Ok(result);
	}
}