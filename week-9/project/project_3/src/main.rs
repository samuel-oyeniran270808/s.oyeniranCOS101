use std::io::Write;

fn main() {
    let commissioner = vec!["Aigbogun Alamba Dauda", "Murtala Afeez Bendu", "Okorocha Calistus Ogbona", "Adewale Jimoh Akanbi", "Osazuwa faith Etieye"];
    let ministry = vec![ "Internal Affairs", "Justice", "Defense", "Power & Steel",  "Petroleum",];
    let zone = vec![ "South West", "North East", "South South", "South West", "South East"];

    let mut record = std::fs::File::create("record.csv").expect("Failed to create file"); 
    record.write_all(b"Student name, Matric no., Department, Level\n")
        .expect("failed to write to record");
   
   for i in 0..commissioner.len() {

    record.write_all(commissioner[i]
        .as_bytes()).expect("failed to write to record");
    record.write_all(b",").unwrap();

    record.write_all(ministry[i].to_string()
        .as_bytes()).expect("failed to write to record");
    record.write_all(b",").unwrap();

    record.write_all(zone[i].to_string()
        .as_bytes()).expect("failed to write to record");
    record.write_all(b" \n").unwrap();
   }
    
}