
use std::fs::OpenOptions;
use std::io::Write;

fn main() {

    let brewery_data = std::fs::File::create("data.csv").expect("create failed");
    let brewery_data = OpenOptions::new().append(true).open("data.csv").expect("cannot open file");
    
    brewery_data.write_all("Lager, Stout, Non-Alcoholic\n".as_bytes()).expect("wriet failed");
    brewery_data.write_all("\n 33Export, Legend, Maltina \n 
    Desperados, Turbo King, Amstel Maltina \n 
    Goldberg, Williams, Malta Gold \n 
    Gulder, , Fayrouz \n 
    Heineken, , , \n 
    Star, , , \n".as_bytes()).expect("failed to append");

}
