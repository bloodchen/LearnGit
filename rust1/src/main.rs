use std::io;

fn main() {

	let mut number = String::new();
	io::stdin().read_line(&mut number)
		.expect("Failed to read line");
    println!("Hello, world! {}", number);
}
