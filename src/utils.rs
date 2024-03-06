use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

use crate::sites::Sites;

pub fn read_file(file_path: String) -> Result<BufReader<File>, String> {
    match File::open(file_path) {
        Ok(file) => Ok(BufReader::new(file)),
        Err(e) => Err(format!("Cloud not open the conf.json. {:?}", e)),
    }
}

pub fn parse_json(reader: BufReader<File>) -> Result<Value, String> {
    match serde_json::from_reader::<BufReader<File>, Value>(reader) {
        Ok(site) => Ok(site),
        Err(e) => Err(format!("Failed to parse json file. {:?}", e)),
    }
}

pub fn convert_site_map(value: Value) -> Option<Sites> {
    if let Some(map) = value.as_object() {
        let mut site_map = HashMap::new();
        let mut names = Vec::new();
        for (key, value) in map.iter() {
            if let Some(v) = value.as_str() {
                site_map.insert(key.clone(), v.to_string());
                names.push(key.clone());
            }
        }
        Some(Sites::new(site_map, names))
    } else {
        None
    }
}
