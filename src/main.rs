use clap::Parser;
use lsplus::{
    out::{default::default, list::list},
    utils::{get_elements_from_path, SortBy},
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

    /// Print contents of directories recursively,
    /// specify maximum recursive depth
    #[arg(short, long, default_value_t = 0)]
    recursive: usize,

    /// Sort elements by parameter
    #[arg(short, long, default_value_t = SortBy::NONE)]
    sort: SortBy,

    /// Path of the directory to list
    #[arg(default_value_t = String::from("."))]
    path: String,
}

fn main() {
    let args = Args::parse();

    let elements = get_elements_from_path(args.path, args.all);

    if args.list {
        list(elements, args.recursive, args.sort);
    } else {
        default(elements);
    }
}
