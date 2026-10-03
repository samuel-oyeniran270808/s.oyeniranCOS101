use std::io;

fn main() {
    println!("Welcome, user");
    println!("          ===================== MENU =====================");
    println!("      ITEM CODE           ITEM                       PRICE");
    println!("      P           Poundo Yam/Edinkaiko Soup         #3,200 \n 
      f           Fried Rice & Chicken              #3,000 \n 
      a           Amala & Ewedu Soup                #2,500 \n 
      e           Eba & Egusi Soup                  #2,000 \n 
      w           White Rice & Stew                 #2,500 \n ");


    let mut total: u32 = 0;
    println!("Welcome, User");

    loop {

        println!("please input the code of your order (press q to exit):");
        let mut code = String::new();
        io::stdin().read_line(&mut code).expect("Faled to read code");
        let code = code.trim();

        let price: u32 = match code {
            "p" => 3_200,
            "f" => 3_000,
            "a" => 2_500,
            "e" => 2_000,
            "w" => 2_500,
            "q" => {

                if total > 10_000 {
                    let discount = total * 10/100;
                    let new_total = total - discount;
                    println!("Your total is {}", total);
                    println!("Because you bought something worth above 10,000, you have a discount of {}", discount); 
                    println!("Your new total is {}", new_total);
                    println!("Thank you for your patronage user, we hope to see you again!");
                }                
                else {
                    println!("Thank you for your patronage user, we hope to see you again!");
                    println!("Your total is {}", total);
                }
                break;
            }
            _=>{
                println!("Invalid input");
                continue;
            }
            
        };

        println!("Input the quantity:");
        let mut quan = String::new();
        io::stdin().read_line(&mut quan).expect("Failed to read quantity");
        let quan: u32 = quan.trim().parse().expect("Failed to convert quantity");

        let cost = price * quan;
        total += cost;
        println!("Cost: {}", cost);


    }
}
