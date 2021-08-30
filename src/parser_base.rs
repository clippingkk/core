use String;
// use chrono::Datetime;

#[derive(Debug)]
pub struct Clipping {
	title: String,
	content: String,
	page_at: String,
	book_id: String,
	created_at: String,
}

#[derive(Debug)]
pub struct ParserBase {
	lang_checker: str,
}

pub trait ParserSolver {
	fn get_title(&self, src: &String) -> String;
	fn get_content(&self, src: &String) -> String;
	fn get_created_at(&self, src: &String) -> String;
	fn get_page_at(&self, src: &String) -> String;
	fn do_parse(&self, src: &String) -> Clipping;
}

impl ParserSolver for ParserBase {
	fn get_title(&self, src: &String) -> String {
		return src.to_string();
	}
	fn get_content(&self, src: &String) -> String {
		return src.to_string();
	}
	fn get_created_at(&self, _: &String) -> String {
		todo!()
	}
	fn get_page_at(&self, _: &String) -> String {
		todo!()
	}
	fn do_parse(&self, _: &String) -> Clipping {
		let c = Clipping {
			title: String::from("s: &sr"),
			content: String::from("s: &sr"),
			page_at: String::from("s: &sr"),
			book_id: String::from("s: &sr"),
			created_at: String::from("s: &sr"),
		};
		return c;
	}
}
