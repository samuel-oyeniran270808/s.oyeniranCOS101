fn main() {
	let p: f64 = 210_000.00;
	let r: f64 = 5.00;
	let t: i32 = 3;

	let amount = p * (1.00 - (r/100.00)).powi(t);
	println!("The television set depreciated by {}", amount);
}