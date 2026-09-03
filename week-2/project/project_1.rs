fn main() {
	
	let p:f32 = 520_000_000.00;

	let r:f32 = 10.00;

	
	let amount:f32 = p * (1.00 + (r / 100.00)) * (1.00 + (r / 100.00)) * (1.00 + (r / 100.00)) * (1.00 + (r / 100.00)) * (1.00 + (r / 100.00));

	let ci = amount - p;

	println!("The simple interest is {}", ci);

/* Here, i noticed that not using the variable would flag an error, so i decided to do the tradition maths of multiplying the 'Principal-Rate' 5 times and i understand that this is not convenient and is lackluster. 
	So i researched on how to use the powers in Rust and i did a separate one just below using it.*/


	let principal:f32 = 520_000_000.00;

	let rate:f32 = 10.00;

	let time:f32 = 5.00;

	let amount_2:f32 = principal * (1.00 + (rate / 100.00)).powf(time);
	let compound_interest = amount_2 - principal;

	println!("The compound interest is {}", compound_interest);

}
