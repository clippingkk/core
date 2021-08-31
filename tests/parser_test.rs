
use crate::parser;

#[cfg(test)]
mod tests {
	use std::path::PathBuf;
	use std::fs::*;
	#[test]
	fn it_works() {
		let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
		d.push("tests/fixtures");

		d.push("clippings_en.txt");

		let data = read_to_string(d.to_str().unwrap()).expect("file");
		let d = parse_from_string(data);

		println!(d);

		assert_eq!("", data)
	}
}