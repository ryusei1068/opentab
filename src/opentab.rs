use fuzzy_select::FuzzySelect;
use std::process::exit;

use crate::sites::Sites;

#[derive(Debug)]
pub struct OpenTab {
    sites: Sites,
}

impl OpenTab {
    pub fn new(sites: Sites) -> Self {
        OpenTab { sites }
    }

    pub fn execute(&self) {
        let site_name = match self.select() {
            Ok(site_name) => site_name,
            Err(_) => exit(1),
        };
        self.open_site(&site_name)
    }

    pub fn select(&self) -> Result<String, String> {
        let options = self.sites.names.clone();
        match FuzzySelect::new()
            .with_prompt("sites")
            .with_options(options)
            .select()
        {
            Ok(selection) => Ok(selection),
            Err(e) => Err(format!("{:?}", e)),
        }
    }

    pub fn open_site(&self, name: &String) {
        if let Some(path) = self.sites.site_map.get(name) {
            match open::that(path) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("An error occurred when opening '{}': {}", path, e);
                    exit(1)
                }
            };
        } else {
            println!("Not found: {}", name);
        }
    }
}
