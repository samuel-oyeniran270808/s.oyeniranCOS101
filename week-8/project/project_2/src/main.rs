use std::io;

fn name_input() -> String {
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("failed to read input");
    let name = name.trim().to_string();
    println!("Thank you for providing us your name, {}", name);
    name
}
fn exp_input() -> i32 {
    loop {
        let mut exp = String::new();
        io::stdin().read_line(&mut exp).expect("failed to read experience");
        match exp.trim().parse::<i32>() {
            Ok(n) if n >= 0 => {
                println!("Please confirm that your yeears of experience is: {} years", n);
                let mut confirmation = String::new();
                io::stdin().read_line(&mut confirmation).expect("Faied to read confrmation");
                if confirmation.trim().to_lowercase() == "y" {
                    println!("Your years of experience is: {} years", n);
                }
                else {
                    continue
                }
                return n;
            }
            _=> println!("Please enter a valid whole number (0 or more):"),
        }
    }
}

fn main() {
    let mut name_vector: Vec<String> = Vec::new();
    let mut exp_vector: Vec<i32> = Vec::new();

    loop {
        println!("Dear User, welcome to Ernst & Young Global Limited");
        println!("One of the rquirement for this jod as a developer is your personal indformation like your name and years of experience.");
        println!("Please, kindly provide that");
        println!("Input your name here:");
        name_vector.push(name_input());

        println!("How many professional working years of experience do you have as a developer?");
        exp_vector.push(exp_input());

        println!("End program(y/n)?");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        if input.trim().to_lowercase() == "y" {
            for (name, exp) in name_vector.iter().zip(exp_vector.iter()) {
                println!("{}: {} years", name, exp);
            }
            break;
        }

    }
    
    if let Some((index, &max_exp)) = exp_vector
    .iter()
    .enumerate()
    .max_by_key(|&(_, exp)| exp)
    {
        println!(
            "The most experienced candidate is: {} with {} years of experience.",
            name_vector[index],
            max_exp
        );
    }
}


