use std::io::Write;

fn main() {
    let names = vec!["Oluchi Mordi", "Addams Aliyu", "Shaina Bolade", "Adekunle Gold", "Blanca Edemoh"];
    let matric_no = vec![ "ACC10211111", "ECO10110101", "CSC10328828", "EEE11020202",  "MEE10202001",];
    let department = vec![ "Accounting", "Economics", "Computer", "Electrical", "Mechanical",];
    let level = vec![ "300", "100", "200", "200", "100"];

    let mut record = std::fs::File::create("record.csv").expect("Failed to create file"); 
    record.write_all(b"Student name, Matric no., Department, Level\n")
        .expect("failed to write to record");
   
   for i in 0..names.len() {

    record.write_all(names[i]
        .as_bytes()).expect("failed to write to record");
    record.write_all(b",").unwrap();

    record.write_all(matric_no[i].to_string()
        .as_bytes()).expect("failed to write to record");
    record.write_all(b",").unwrap();

    record.write_all(department[i].to_string()
        .as_bytes()).expect("failed to write to record");
    record.write_all(b",").unwrap();

    record.write_all(level[i].to_string()
        .as_bytes()).expect("failed to write to record");
    record.write_all(b" \n").unwrap();
   }
    
}
