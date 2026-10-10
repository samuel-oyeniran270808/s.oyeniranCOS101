use std::io;

fn T_area(T_height: f64, b1: f64, b2: f64) -> f64 {
    T_height / 2.0 * (b1 + b2)
}

fn R_area(diag1: f64, diag2: f64) -> f64 {
    1.0 / 2.0 * diag1 * diag2
}

fn P_area(base: f64, alt: f64) -> f64 {
    base * alt
}

fn Cube_Surface_area(side: f64) -> f64 {
    6.0 * side * side
}

fn Cylinder_volume(r: f64, C_height: f64) -> f64 {
    3.142 * r * r * C_height
}

fn main() {
    println!("Welcome, User");
    println!("What would you like to do? \n
        T       Area of Trapezium
        R       Area of Rhombus
        P       Area of Parallelogram
        S       Surface area of cube
        C       Volume of cylinder
    ");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice)
        .expect("Failed to read choice");

    if choice.trim() == "T" || choice.trim() == "t" {
        println!("Input the height of trapezium");
        let mut T_height = String::new();
        io::stdin().read_line(&mut T_height).expect("Failed to read height");

        println!("Input the first base of trapezium");
        let mut b1 = String::new();
        io::stdin().read_line(&mut b1).expect("Failed to read base");

        println!("Input the second base of trapezium");
        let mut b2 = String::new();
        io::stdin().read_line(&mut b2).expect("Failed to read base");

        let T_height: f64 = T_height.trim().parse().expect("Invalid number");
        let b1: f64 = b1.trim().parse().expect("Invalid number");
        let b2: f64 = b2.trim().parse().expect("Invalid number");

        println!("Area of trapezium = {}", T_area(T_height, b1, b2));
    }

    else if choice.trim() == "R" || choice.trim() == "r" {
        println!("Input the first diagonal of rhombus");
        let mut diag1 = String::new();
        io::stdin().read_line(&mut diag1).expect("Failed to read diagonal");

        println!("Input the second diagonal of rhombus");
        let mut diag2 = String::new();
        io::stdin().read_line(&mut diag2).expect("Failed to read diagonal");

        let diag1: f64 = diag1.trim().parse().expect("Invalid number");
        let diag2: f64 = diag2.trim().parse().expect("Invalid number");

        println!("Area of rhombus = {}", R_area(diag1, diag2));
    }

    else if choice.trim() == "P" || choice.trim() == "p" {
        println!("Input the base of parallelogram");
        let mut base = String::new();
        io::stdin().read_line(&mut base).expect("Failed to read base");

        println!("Input the altitude of parallelogram");
        let mut alt = String::new();
        io::stdin().read_line(&mut alt).expect("Failed to read altitude");

        let base: f64 = base.trim().parse().expect("Invalid number");
        let alt: f64 = alt.trim().parse().expect("Invalid number");

        println!("Area of parallelogram = {}", P_area(base, alt));
    }

    else if choice.trim() == "S" || choice.trim() == "s" {
        println!("Input the side of the cube");
        let mut side = String::new();
        io::stdin().read_line(&mut side).expect("Failed to read side");

        let side: f64 = side.trim().parse().expect("Invalid number");

        println!("Surface area of cube = {}", Cube_Surface_area(side));
    }

    else if choice.trim() == "C" || choice.trim() == "c" {
        println!("Input the radius of cylinder");
        let mut r = String::new();
        io::stdin().read_line(&mut r).expect("Failed to read radius");

        println!("Input the height of cylinder");
        let mut C_height = String::new();
        io::stdin().read_line(&mut C_height).expect("Failed to read height");

        let r: f64 = r.trim().parse().expect("Invalid number");
        let C_height: f64 = C_height.trim().parse().expect("Invalid number");

        println!("Volume of cylinder = {}", Cylinder_volume(r, C_height));
    }

    else {
        println!("Invalid choice");
    }
}