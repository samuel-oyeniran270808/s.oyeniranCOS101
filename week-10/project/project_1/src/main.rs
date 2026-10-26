struct Laptops {
    brand: Vec<String>,
    quantity: Vec<u32>,
    price: Vec<u32>
}
impl Laptops {
    fn total(&mut self) -> u32 {
        let mut sum = 0;
        for i in self.price.iter_mut() {
            sum += *i * 3;
        }
        sum
     }
}

fn main() {
    let mut small = Laptops {
        brand: vec!["Hp".to_string(), "IBM".to_string(), "Toshiba".to_string(), "Dell".to_string()],
        quantity: vec![10, 6, 10, 4],
        price: vec![650_000, 755_000, 550_000, 850_000]
    };

    println!("Customer purchased HP, IBM, Toshiba and Dell, 3 of each. Total = {:?}", small.total());
}
