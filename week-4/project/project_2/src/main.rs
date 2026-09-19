use std::io;

fn main() {


    // This is for experience
    println!("Please, input your years of experience: ");
    let mut exp = String::new(); // This is what makes the user input whatever they want to

    io::stdin() // This is the input functionality
        .read_line(&mut exp)// Reads what the user inputs and stores it in the exp variable
        .expect("Not a valid input for your expeience");// Prints a user frinedly error if an unaccepted value is inputted
    let exp: u8 = exp
        .trim() //Removes whitespace
        .parse() //Converts String to integer of 8 bits
        .expect("Numeric input expected");
    println!("Your experience is {} years", exp);


    //This is for age
    println!("Please, input your age: ");
    let mut age = String::new();

    io::stdin()
        .read_line(&mut age)
        .expect("Not a valid input for your age");
    let age: u8 = age
        .trim()
        .parse()
        .expect("Numeric input expected(2)");

        // I used 'const' here because this values are not going to change at all
    const A1: i64 = 1_560_000;
    const A2: i64 = 1_480_000;
    const A3: i64 = 1_300_000; 
    const A4: i32 = 100_000;
  
  /*
 At first i tried doing something like: if exp >= 3 && age >= 40 {
    
 } 
 else if exp >= 3 && age >= 30 && age < 40{
    
 } 
 Howver, i noticed i was getting lots of error especially in this fist else if condition so i decided "Why not create a condition of exp > 3 first?"
 So i did this instead. Created a condition that says experience is > 3 and put all other "sub-conditions" in it and the else if for exp < 3 and 
 when the user inputs something strange that may have escaped the ".expect()" i put above or if te user doesn,t have any experience at all or if they input 0 as age.

 One last thing, i used ELSE IF instead of just using IF because i wanted to see how it would look and work. Was just exerimenting. 
  */

    if exp > age {
                println!("Invalid input");
                return;
        }
    if exp >= 3 {
             if age >= 40 {
            println!("Your annual incentive is {} considering your age and years of experince", A1);
            }
            else if age >= 30 && age < 40 {
                println!("Your annual incentive is {} considering your age and years of experince", A2);
            }
            else if age < 29  && age >= 18 {
                println!("Your annual incentive is {} considering your age and years of experince", A3);
            }
            else if age < 18 {
                println!("You do not get an incentive because you are not of working age");
            }
        }
    else if exp < 3 {
            println!("Your annual incentive is {} considering your age and years of experince", A4);
        }
    else if exp <= 3 && age < 18 {
            println!("You do not get an incentive because you are not of working age");
        }
    else {
        println!("Sorry, you are not eligible to any incentive!😭");
    }
}
