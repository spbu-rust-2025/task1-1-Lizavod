use std::io;

fn main(){
	let mut input = String::new();
	io::stdin()
		.read_line(&mut input)
		.expect("Couldn't read the line");
	let numbers: Vec<i32> = input
		.split_whitespace()
		.map(|s| s.parse().expect("Enter the correct numbers"))
		.collect();
	if numbers.len() < 2 {
		println!("You must enter at least 2 numbers");
		return;
	}
	println!("{}", numbers[0]+numbers[1]);
}