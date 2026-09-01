fn main() {
	let toshiba: f64 = 450_000.00;
	let mac: f64 = 1_500_000.00;
	let hp: f64 = 750_000.00;
	let dell: f64 = 2_850_000.00;
	let acer: f64 = 250_000.00;
	let amount_of_product: f64 = 10.00; 
	let sum = (2.00 * toshiba) + mac + (3.00 * hp) + (3.00 * dell) + acer;
	println!("The sum of this sales record is {}", sum);

	let average: f64 = sum /  amount_of_product;
	println!("The average is {}", average);
}