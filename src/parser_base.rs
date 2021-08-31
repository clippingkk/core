use chrono::prelude::*;
use regex::Regex;
use String;

#[derive(Debug)]
pub struct Clipping {
	pub title: String,
	pub content: String,
	pub page_at: String,
	pub book_id: String,
	pub created_at: DateTime<Utc>,
}

#[derive(Debug)]
pub struct ParserBase {
	pub lang_checker: String,
	pub page_at_str: String,
	pub location_regexp: Regex,
	pub date_transformer: fn(String) -> String,
}

pub trait ParserSolver {
	fn get_title(&self, src: &str) -> String;
	fn get_content(&self, src: &str) -> String;
	fn get_created_at(&self, src: &str) -> DateTime<Utc>;
	fn get_page_at(&self, src: &str) -> String;
	fn do_parse(&self, src: Vec<&str>) -> Clipping;
}

impl ParserSolver for ParserBase {
	fn get_title(&self, src: &str) -> String {
		return src.to_string();
	}
	fn get_content(&self, src: &str) -> String {
		let content = src.trim();
		if content.len() == 0 {
			return "".to_string();
		}
		let regexHead = Regex::new(r"^(，|。|！|@|【】|——)+").unwrap();
		let regexTail = Regex::new(r"(，|。|！|@|【】|——)+$").unwrap();
		let content2 = &regexHead.replace_all(content, "");
		let content3 = &regexTail.replace_all(content2, "");
		return content3.trim().to_string();
	}
	fn get_created_at(&self, src: &str) -> DateTime<Utc> {
		let arr: Vec<&str> = src.split("|").collect();
		let theDateStr = arr.last().unwrap();
		let dist = (self.date_transformer)(String::from(theDateStr.clone()));

		let datetime = dist.parse::<DateTime<Utc>>();
		match datetime {
			Ok(v) => return v,
			Err(e) => return Utc.timestamp(0, 0),
		}
	}
	fn get_page_at(&self, src: &str) -> String {
		let arr: Vec<&str> = src.split("|").collect();
		let theLocationStr = arr.first().unwrap();
		let m = self.location_regexp.shortest_match(theLocationStr);

		let c = self.location_regexp.captures(theLocationStr);
		if c.is_none() {
			return "".to_string();
		}
		let cc = c.unwrap();
		let pageAtStr = cc.get(cc.len() - 1).unwrap().as_str();
		let mut p = pageAtStr.replace(&self.page_at_str, "");
		if p.contains("-") && !p.starts_with("#") {
			p = String::from("#") + &p
		}
		return p.trim().to_string();
	}
	fn do_parse(&self, src: Vec<&str>) -> Clipping {
		let mut c = Clipping {
			title: String::from(""),
			content: String::from(""),
			page_at: String::from(""),
			book_id: String::from(""),
			created_at: Utc::now(),
		};

		for (idx, row) in src.iter().enumerate() {
			match idx % 5 {
				0 => c.title = self.get_title(row),
				1 => {
					c.page_at = self.get_page_at(row);
					c.created_at = self.get_created_at(row);
				}
				2 => c.content = self.get_content(row),
				3 => {
					println!("{:?}", c);
				},
				_ => {
					println!("unknown")
				}
			}
		}

		return c;
	}
}
