fn main() {
	let len = 10.0;
	let width = 5.0;

	let area = calculate_area(len, width);

	println!("The area of a rectangle with length {} and width {} is {}", len, width, area);
}

fn calculate_area(length: f64, width: f64) -> f64 {
	length * width
}
