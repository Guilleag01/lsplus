use clap::Parser;
use lsplus::{
    element::Element,
    out::{default::default, list::list},
};
use std::fs;

// Needs to be defined in main
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Show hidden files
    #[arg(short, long, default_value_t = false)]
    all: bool,

    /// Print as a list
    #[arg(short, long, default_value_t = false)]
    list: bool,

    /// Path of the directory to list
    #[arg(default_value_t = String::from("."))]
    path: String,
}

fn main() {
    let args = Args::parse();

    let paths = fs::read_dir(args.path).unwrap();

    let _max_width = 50;

    let elements: Vec<Element> = paths
        .map(|e| Element::new(e.unwrap().path().to_str().unwrap()))
        .filter(|element| args.all || !element.get_name().starts_with('.'))
        .collect();

    if args.list {
        list(elements);
    } else {
        default(elements);
    }
}
