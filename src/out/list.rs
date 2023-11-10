use crate::element::Element;
use crate::utils::{get_size_string, get_string_length, pad_string, system_time_to_string};

pub fn list(mut elements: Vec<Element>) {
    elements.sort_unstable_by_key(|a: &Element| a.get_name());
    let width = term_size::dimensions().unwrap().0;
    //  ╭──────────────╼ File name ╾──────────────┬─╼ Size ╾─┬──╼ Creation ╾──╮
    //  │ some_example_file                       │ 420.69 G │ 01-01-70 00:00 │
    //  ╰─────────────────────────────────────────┴──────────┴────────────────╯
    let mut name_max_len = 0;
    for e in &elements {
        let length = get_string_length(e.to_string().as_str());
        if length > name_max_len {
            name_max_len = length;
        }
    }

    let name_length = name_max_len
        .max(14 + (elements.len() as f32).log10() as usize)
        .max(13)
        .min(width - 30);

    print_header(name_length);
    print_elements(&elements, name_length);
    print_footer(&elements, name_length);
}

fn print_header(name_length: usize) {
    print!("╭");
    for _ in 0..((name_length - 13) as f32 / 2.0).floor() as usize {
        print!("─");
    }
    print!("╼ File name ╾");
    for _ in 0..((name_length - 13) as f32 / 2.0).ceil() as usize {
        print!("─");
    }
    println!("┬─╼ Size ╾─┬──╼ Creation ╾──╮");
}

fn print_elements(elements: &Vec<Element>, name_length: usize) {
    for e in elements {
        let str_len = get_string_length(e.get_name().as_str());
        print!("│");
        if get_string_length(e.to_string().as_str()) > name_length {
            print!(
                "{}",
                pad_string(
                    e.to_string().as_str()[..=name_length].to_string(),
                    name_length,
                    true
                )
            );
            print!("│");
            print!("{}", pad_string(get_size_string(e.get_size()), 10, false));
            print!("│");
            print!(" {} ", system_time_to_string(e.get_creation()));
            println!("│");

            for i in 1..(str_len / (name_length - 5) + 1) {
                print!(
                    "│   {}",
                    pad_string(
                        e.get_name().as_str()
                            [((name_length - 5) * i)..((name_length - 5) * (i + 1)).min(str_len)]
                            .to_string(),
                        name_length - 3,
                        true
                    )
                );

                println!("│          │                │");
            }
        } else {
            print!("{}", pad_string(e.to_string(), name_length, true));
            print!("│");
            print!("{}", pad_string(get_size_string(e.get_size()), 10, false));
            print!("│");
            print!(" {} ", system_time_to_string(e.get_creation()));
            println!("│");
        }
    }
}

fn print_footer(elements: &Vec<Element>, name_length: usize) {
    let num_elements = elements.len();
    let num_elements_len = (num_elements as f32).log10() as usize;
    let name_length_fixed = name_length - (num_elements_len + 14);
    print!("╰");
    for _ in 0..((name_length_fixed) as f32 / 2.0).floor() as usize {
        print!("─");
    }
    print!("╼ {} Elements ╾", num_elements);
    for _ in 0..((name_length_fixed) as f32 / 2.0).ceil() as usize {
        print!("─");
    }
    // for _ in 0..(name_length - (num_elements_len + 15)) {
    //     print!("─");
    // }
    println!("┴──────────┴────────────────╯");
}
