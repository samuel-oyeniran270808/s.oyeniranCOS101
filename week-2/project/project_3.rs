fn main() {
	let p: f32 = 210_000.00;
	let r: f32 = 5.00;
	let t: f32 = 3.00;

	let amount = p * (1.00 - (r/100.00)).powf(t);
	println!("The television set depreciated by {}", amount);
}