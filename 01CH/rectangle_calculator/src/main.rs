use std::io;

fn main() {
	println!("Rectangle area calculator.\nEnter the length:");
	let mut input = String::new();

	io::stdin().read_line(&mut input).expect("Failed to read line");


	let len: f64 = input.trim().parse().expect("Not a valid number");

	println!("Enter the width:");
	input.clear();
	io::stdin().read_line(&mut input).expect("Failed to read line");


	let width: f64 = input.trim().parse().expect("Not a valid number");


	let area = calculate_area(len, width);

	println!("The area of a rectangle with length {} and width {} is {}", len, width, area);
}

fn calculate_area(length: f64, width: f64) -> f64 {
	length * width
}
