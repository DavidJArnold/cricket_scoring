mod cricsheet_lib;
use cricsheet_lib::Cricsheet;
use std::{fs::File, io::Read};

fn main() {
    let filename = "examples/all_matches";
    let num_files = std::fs::read_dir(filename).unwrap().count();
    let mut read_files = 0;
    for file in std::fs::read_dir(filename).unwrap() {
        let x = file.unwrap();
        if x.path().to_str().unwrap().ends_with("txt") {
            continue;
        }
        // println!("{:?}", x.path());
        let mut data = String::new();
        let mut file = File::open(x.path()).unwrap();
        // let mut file = File::open("examples/game.json").expect("");
        let _ = &file.read_to_string(&mut data);
        let json: Result<serde_json::Value, serde_json::Error> = serde_json::from_str(&data);
        if json.is_err() { println!("JSON Error: {:?}", x.path()) };
        let cricsheet: Result<Cricsheet, serde_json::Error> = serde_json::from_str(&data);
        if cricsheet.is_err() { println!("DATA Error: {:?}", x.path()) };
        read_files += 1;
        if read_files % 100 == 0 {
            println!("{read_files}/{num_files}");
        }
    }
}
