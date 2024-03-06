use std::collections::HashMap;

#[derive(Debug)]
pub struct Sites {
    pub site_map: HashMap<String, String>,
    pub names: Vec<String>,
}

impl Sites {
    pub fn new(site_map: HashMap<String, String>, names: Vec<String>) -> Self {
        Sites { site_map, names }
    }
}
