mod parser;
mod err;

fn main() {
    let src = String::from("hello \n world\nfrom\nsome\nfile");
    let result = parser::parse_from_string(src).unwrap();
    println!("Hello, world! {:?}", result.first());
}
