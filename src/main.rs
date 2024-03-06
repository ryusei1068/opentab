use std::env;
use std::process::exit;

mod opentab;
use opentab::OpenTab;
mod sites;
mod utils;

fn main() {
    let file_path = env::var("OPENTAB_CONF").expect("OPENTAB_CONF is not defined");

    let file = match utils::read_file(file_path) {
        Ok(f) => f,
        Err(e) => {
            println!("{:?}", e);
            exit(1)
        }
    };

    let value = match utils::parse_json(file) {
        Ok(v) => v,
        Err(e) => {
            println!("{:?}", e);
            exit(1)
        }
    };

    let opentab = if let Some(site) = utils::convert_site_map(value) {
        OpenTab::new(site)
    } else {
        exit(1)
    };

    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        opentab.open_site(&args[1]);
    } else {
        opentab.execute();
    }
}
