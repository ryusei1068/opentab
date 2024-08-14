use crate::sites::Sites;
use fuzzy_select::{ContentStyle, FuzzySelect, Stylize, Theme};
use std::process::exit;

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
        let theme = Theme {
            selected_indicator: '>'.blue().bold(),
            indicator: ' '.reset(),
            selected_text: ContentStyle::new().green().bold(),
            text: ContentStyle::new(),
            selected_highlight: ContentStyle::new().black().on_yellow(),
            highlight: ContentStyle::new().dark_yellow().on_yellow(),
        };

        let options = self.sites.names.clone();
        match FuzzySelect::new()
            .with_theme(theme)
            .with_prompt(">")
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
