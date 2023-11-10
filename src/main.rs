use clap::Parser;
use lsplus::{
    out::{default::default, list::list},
    utils::get_elements_from_path,
};

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

    #[arg(short, long, default_value_t = 0)]
    recursive: usize,

    /// Path of the directory to list
    #[arg(default_value_t = String::from("."))]
    path: String,
}

fn main() {
    let args = Args::parse();

    let elements = get_elements_from_path(args.path, args.all);

    // let paths = fs::read_dir(args.path).unwrap();

    // let elements: Vec<Element> = paths
    //     .map(|e| Element::new(e.unwrap().path().to_str().unwrap()))
    //     .filter(|element| args.all || !element.get_name().starts_with('.'))
    //     .collect();

    if args.list {
        list(elements, args.recursive);
    } else {
        default(elements);
    }
}
