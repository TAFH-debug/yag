mod configs;

use std::collections::HashMap;
use configs::*;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, default_value = "None")]
    name: String,
    #[clap(short, long, default_value_t = 1)]
    count: i16
}

fn main() {
    let mut configs: HashMap<String, String> = HashMap::new();
    let args = Args::parse();
    if !std::path::Path::new(get_config_path().as_str()).exists() {
        create_configs();
    }
    match check_configs() {
        Ok(_) => (),
        Err(n) => {/*Logging and showing error text*/}
    }
    load_configs(&mut configs);
    println!("{:?}", configs);
}
