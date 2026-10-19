use std::fs;
use std::io::Write;

fn main() {
    let mut file = std::fs::File::create("data.txt").expect("create failed");
    file.write_all("This file has been deleted\n"
        .as_bytes()).expect("create failed");
    fs::remove_file("data.txt").expect("could not remove file");
    println!("file is removed");
}
