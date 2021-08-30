use crate::err::CKError;
use crate::parser_base::Clipping;
use std::result::Result;

const lang_en_checker: &str = "Your Highlight on";
const lang_zh_checker: &str = "您在";

pub fn parse_from_string(src: String) -> Result<Vec<Clipping>, CKError> {
	let mut lines: Vec<Vec<&str>> = Vec::new();
	let plain_lines: Vec<&str> = src.split("\n").collect();

	if src.contains(lang_en_checker) {
		// is eng
	} else if src.contains(lang_zh_checker) {

	} else {
		return Err(CKError::new("hello"))
	}



	let mut cursor:Vec<&str> = Vec::new();
	for l in plain_lines {
		let line = l.trim();
		if line.contains("========") {
			lines.push(cursor.clone());
			cursor.clear();
			continue;
		}
		if line.is_empty() {
			continue;
		}
		cursor.push(line);
	}

    let mut result: Vec<Clipping> = Vec::new();
    let c = Clipping {
        title: String::from("s: &sr"),
        content: String::from("s: &sr"),
        page_at: String::from("s: &sr"),
        book_id: String::from("s: &sr"),
        created_at: String::from("s: &sr"),
    };
    result.push(c);
    Ok(result)
}