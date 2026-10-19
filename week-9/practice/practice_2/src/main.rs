use std::io::Read;
use std::io::Write;

fn main() {
    let mut message = std::fs::File::create("welcome_message.txt").expect("create failed");
    message.write_all("Welcome dear User\n"
        .as_bytes()).expect("write failed");
    let mut file = std::fs::File::open("welcome_message.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}
