use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use serde_json::Value;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::process::exit;

#[derive(Debug)]
struct OpenTab {
    sites: Sites,
}

impl OpenTab {
    fn new(sites: Sites) -> Self {
        OpenTab { sites }
    }

    fn execute(&self) {
        let site_name = match self.select() {
            Ok(site_name) => site_name,
            Err(_) => exit(1),
        };
        self.open_site(site_name)
    }

    fn select(&self) -> Result<&String, String> {
        match FuzzySelect::with_theme(&ColorfulTheme::default())
            .with_prompt("sites")
            .items(&self.sites.names)
            .interact()
        {
            Ok(selection) => Ok(&self.sites.names[selection]),
            Err(e) => Err(format!("{:?}", e)),
        }
    }

    fn open_site(&self, name: &String) {
        if let Some(path) = self.sites.site_map.get(name) {
            match open::that(path) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("An error occurred when opening '{}': {}", path, e);
                    exit(1)
                }
            };
        };
    }
}

#[derive(Debug)]
struct Sites {
    site_map: HashMap<String, String>,
    names: Vec<String>,
}

fn read_file(file_path: String) -> Result<BufReader<File>, String> {
    match File::open(file_path) {
        Ok(file) => Ok(BufReader::new(file)),
        Err(e) => Err(format!("Cloud not open the conf.json. {:?}", e)),
    }
}

fn parse_json(reader: BufReader<File>) -> Result<Value, String> {
    match serde_json::from_reader::<BufReader<File>, Value>(reader) {
        Ok(site) => Ok(site),
        Err(e) => Err(format!("Failed to parse json file. {:?}", e)),
    }
}

fn convert_site_map(value: Value) -> Option<Sites> {
    if let Some(map) = value.as_object() {
        let mut site_map = HashMap::new();
        let mut names = Vec::new();
        for (key, value) in map.iter() {
            if let Some(v) = value.as_str() {
                site_map.insert(key.clone(), v.to_string());
                names.push(key.clone());
            }
        }
        Some(Sites {
            site_map: site_map,
            names: names,
        })
    } else {
        None
    }
}

fn main() {
    let file_path = env::var("OPENTAB_CONF").expect("OPENTAB_CONF is not defined");

    let file = match read_file(file_path) {
        Ok(f) => f,
        Err(e) => {
            println!("{:?}", e);
            exit(1)
        }
    };

    let value = match parse_json(file) {
        Ok(v) => v,
        Err(e) => {
            println!("{:?}", e);
            exit(1)
        }
    };

    let opentab = if let Some(site) = convert_site_map(value) {
        OpenTab::new(site)
    } else {
        exit(1)
    };

    opentab.execute();
}
