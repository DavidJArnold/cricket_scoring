mod cricsheet_lib;
use cricsheet_lib::Cricsheet;
use serde_json::Value;
use std::{collections::HashMap, fs::File, io::Read};


fn main() {
    // record field with error, error description and each record which raises the error
    let mut errors: HashMap<[String; 3], Vec<String>> = HashMap::new();

    // directory containing json game files
    let filename = "examples/all_matches";

    // total number of files in directory (includes non-json files)
    let num_files = std::fs::read_dir(filename).unwrap().count();
    
    // number of files parsed
    let mut read_files = 0;

    for file in std::fs::read_dir(filename).unwrap() {
        let x = file.unwrap();
        if !x.path().to_str().unwrap().ends_with("json") {
            // skip non-JSON files
            continue;
        }

        // load data
        let mut data = String::new();
        let mut file = File::open(x.path()).unwrap();
        let _ = &file.read_to_string(&mut data);

        // read into a Value
        let json: Result<serde_json::Value, serde_json::Error> = serde_json::from_str(&data);
        // parse into Cricsheet object
        let cricsheet: Result<Cricsheet, serde_json::Error> = serde_json::from_value(json.unwrap());

        if cricsheet.is_err() {
            // get information about the error
            let err_path: Result<Cricsheet, _> = serde_path_to_error::deserialize(&mut serde_json::Deserializer::from_str(&data));
            let series = match serde_json::from_str::<Value>(&data).unwrap().get("info").unwrap().get("event") {
                Some(event) => event.get("name").unwrap().to_string(),
                None => String::new(),
            };
            let game = x.path().file_stem().unwrap().to_str().unwrap().to_string();
            let field: String = err_path.err().unwrap().path().to_string().chars().filter(|&c| !c.is_digit(10)).collect();
            let msg = format!("{}", cricsheet.err().unwrap()).chars().filter(|&c| !c.is_digit(10)).collect();
            let key = [field, series, msg];
            // load into errors or extend existing value
            let val = errors.get_mut(&key);
            match val {
                Some(v) => { v.push(game); },
                None => {errors.insert(key, vec![game]);}
            };
        };

        // progress tracking
        read_files += 1;
        if read_files % 100 == 0 {
            println!("{read_files}/{num_files}");
        }
    }
    // print summary
    for (key, val) in errors.iter() {
        println!("{} {} {}", key.get(0).unwrap(), key.get(1).unwrap(), val.len());
    }
}
