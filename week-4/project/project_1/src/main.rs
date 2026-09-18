use std::io;

fn main() {

    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();
    

    println!("Please, input a value for 'a': ");
    io::stdin().read_line(&mut a).expect("Not a valid input for a");
    println!("You inputted {} for a!", a);

    println!("Please, input a value for 'b': ");
    io::stdin().read_line(&mut b).expect("Not a valid input for b");
    println!("You inputted {} for b!", b);

    println!("Please, input a value for 'c': ");
    io::stdin().read_line(&mut c).expect("Not a valid input for a constant");
    println!("You inputted {} for c!", c);

    let a: f32 = a.trim().parse().expect("Invalid input, please input a number"); 
    let b: f32 = b.trim().parse().expect("Invalid second input, Please input a number");
    let c: f32 = c.trim().parse().expect("Invalid input, please input a number");
    let mut d: f32 = b * b - 4.0 * a * c;


    if d > 0.0 {
        let x1 = (-b + d.sqrt()) / (2.0 * a);
        let x2 = (-b - d.sqrt()) / (2.0 * a);
        println!("x = {} and x = {}", x1, x2);
    }
    if d == 0.0 {
        let x3 = -b / (2.0 * a);
        println!("x = {}", x3);
    }
    if d < 0.0 {
        println!("x doesn't have a real root!");
    }
}
