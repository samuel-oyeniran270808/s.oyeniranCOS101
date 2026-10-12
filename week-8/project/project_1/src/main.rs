use std::io;

fn main() {
    let levels = vec![
        "APS 1-2",
        "APS 3-5",
        "APS 5-8",
        "EL1 8-10",
        "EL2 10-13",
        "SES",
    ];

    let office_admin = vec![
        "Intern",
        "Administrator",
        "Senior Administrator",
        "Office Manager",
        "Director",
        "CEO",
    ];

    let academic = vec![
        "Not specified",
        "Research Assistant",
        "PhD Candidate",
        "Post-Doc Researcher",
        "Senior Lecturer",
        "Dean",
    ];

    let lawyer = vec![
        "Paralegal",
        "Junior Associate",
        "Associate",
        "Senior Associate 1-2",
        "Senior Associate 3-4",
        "Partner",
    ];

    let teacher = vec![
        "Placement",
        "Classroom Teacher",
        "Senior Teacher",
        "Leading Teacher",
        "Deputy Principal",
        "Principal",
    ];

    println!("=== APS LEVEL CHECKER ===");
    println!("Choose your profession:");
    println!("1. Office Admin");
    println!("2. Academic");
    println!("3. Lawyer");
    println!("4. Teacher");

    let mut profession = String::new();

    io::stdin()
        .read_line(&mut profession)
        .expect("Failed to read input");

    let profession = profession.trim();

    let titles = match profession {
        "1" => &office_admin,
        "2" => &academic,
        "3" => &lawyer,
        "4" => &teacher,
        _ => {
            println!("Invalid profession selected.");
            return;
        }
    };

    println!("Enter your years of experience:");

    let mut experience = String::new();

    io::stdin()
        .read_line(&mut experience)
        .expect("Failed to read input");

    let exp: u32 = match experience.trim().parse() {
        Ok(value) => value,
        Err(_) => {
            println!("Please enter a valid whole number.");
            return;
        }
    };

    let index = match exp {
        0..=2 => 0,
        3..=5 => 1,
        6..=8 => 2,
        9..=10 => 3,
        11..=13 => 4,
        _ => 5,
    };

    println!("\n=== RESULT ===");
    println!("Experience: {} years", exp);
    println!("APS Level: {}", levels[index]);
    println!("Job Title: {}", titles[index]);
}