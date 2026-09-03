fn main() {
	let toshiba: f32 = 450_000.00;
	let mac: f32 = 1_500_000.00;
	let hp: f32 = 750_000.00;
	let dell: f32 = 2_850_000.00;
	let acer: f32 = 250_000.00;

	let amount_toshiba: f32 = 2.00;
	let amount_hp: f32 = 3.00;
	let amount_dell: f32 = 3.00;
	let amount_mac: f32 = 1.00;
	let amount_acer: f32 = 1.00;

	let amount_of_product: f32 = amount_toshiba + amount_hp + amount_mac + amount_acer + amount_dell; 
	let sum = (amount_toshiba * toshiba) + (amount_mac * mac) + (amount_hp * hp) + (amount_dell * dell) + (amount_acer * acer);


	println!("The sum of this sales record is {}", sum);

	let average: f32 = sum /  amount_of_product;
	println!("The average is {}", average);
}