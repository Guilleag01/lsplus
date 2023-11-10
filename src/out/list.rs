use crate::element::{Element, TypeOfFile};
use crate::utils::{
    get_elements_from_path, get_size_string, get_string_length, pad_string, system_time_to_string,
};

pub fn list(mut elements: Vec<Element>, recursive_limit: usize) {
    elements.sort_unstable_by_key(|a: &Element| a.get_name());
    let width = term_size::dimensions().unwrap().0;
    //  ╭──────────────╼ File name ╾──────────────┬─╼ Size ╾─┬──╼ Creation ╾──╮
    //  │ some_example_file                       │ 420.69 G │ 01-01-70 00:00 │
    //  ╰─────────────────────────────────────────┴──────────┴────────────────╯

    let name_max_len = get_max_width(&elements, recursive_limit, 0);

    let name_length = name_max_len
        .max(14 + (elements.len() as f32).log10() as usize)
        .max(13)
        .min(width - 30);

    print_header(name_length);
    print_elements(&elements, name_length, recursive_limit, 0, &Vec::new());
    print_footer(&elements, name_length);
}

fn print_header(name_length: usize) {
    print!("╭");
    for _ in 0..((name_length - 12) as f32 / 2.0).floor() as usize {
        print!("─");
    }
    print!("╼ File name ╾");
    for _ in 0..((name_length - 12) as f32 / 2.0).ceil() as usize {
        print!("─");
    }
    println!("┬─╼ Size ╾─┬──╼ Creation ╾──╮");
}

fn print_elements(
    elements: &Vec<Element>,
    name_length: usize,
    recursive_limit: usize,
    current_depth: usize,
    is_last_element: &[bool],
) {
    let mut new_is_last_element = is_last_element.to_owned();

    // println!("{:?}", is_last_element);

    for (i, e) in elements.iter().enumerate() {
        let str_len = get_string_length(e.get_name().as_str());
        print!("│ ");
        if get_string_length(e.to_string().as_str()) > name_length {
            let mut e_string = String::new();
            if current_depth > 0 {
                for &is_last in &new_is_last_element[1..] {
                    if is_last {
                        e_string.push_str("  ");
                    } else {
                        e_string.push_str("│ ");
                    }
                }
                if i == elements.len() - 1 {
                    e_string.push_str("╰─");
                } else {
                    e_string.push_str("├─");
                }
            }
            e_string.push_str(e.to_string().as_str());
            print!(
                "{}",
                pad_string(
                    e_string.as_str()[..=name_length].to_string(),
                    name_length - 2,
                    true
                )
            );
            print_size_and_creation_date(e);
            let mut e_name = String::new();
            if current_depth > 0 {
                for _ in 0..current_depth {
                    e_name.push_str("  ");
                }
            }
            e_name.push_str(e.get_name().as_str());
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
            let mut e_string = String::new();
            if current_depth > 0 {
                for &is_last in &new_is_last_element[1..] {
                    if is_last {
                        e_string.push_str("  ");
                    } else {
                        e_string.push_str("│ ");
                    }
                }

                if i == elements.len() - 1 {
                    e_string.push_str("╰─");
                } else {
                    e_string.push_str("├─");
                }
            }
            e_string.push_str(e.to_string().as_str());
            print!("{}", pad_string(e_string, name_length, true));
            print_size_and_creation_date(e)
        }

        if e.get_file_type() == TypeOfFile::Dir && current_depth < recursive_limit {
            let dir_path = e.get_path_string();
            new_is_last_element.push(i == elements.len() - 1);
            print_elements(
                &get_elements_from_path(dir_path, true),
                name_length,
                recursive_limit,
                current_depth + 1,
                &new_is_last_element,
            );
            new_is_last_element.pop();
            // println!("{:?}", is_last_element);
        }
    }
}

fn print_size_and_creation_date(e: &Element) {
    print!("│");
    if e.get_file_type() == TypeOfFile::Dir {
        print!("          ");
    } else {
        print!("{}", pad_string(get_size_string(e.get_size()), 10, false));
    }
    print!("│");
    print!(" {} ", system_time_to_string(e.get_creation()));
    println!("│");
}

fn print_footer(elements: &Vec<Element>, name_length: usize) {
    let num_elements = elements.len();
    let num_elements_len = (num_elements as f32).log10() as usize;
    let name_length_fixed = name_length - (num_elements_len + 13);
    print!("╰");
    for _ in 0..((name_length_fixed) as f32 / 2.0).floor() as usize {
        print!("─");
    }
    print!("╼ {} Elements ╾", num_elements);
    for _ in 0..((name_length_fixed) as f32 / 2.0).ceil() as usize {
        print!("─");
    }
    println!("┴──────────┴────────────────╯");
}

fn get_max_width(elements: &Vec<Element>, recursive_limit: usize, current_depth: usize) -> usize {
    let mut name_max_len = 0;
    for e in elements {
        let mut length = get_string_length(e.to_string().as_str()) + current_depth * 2;
        if e.get_file_type() == TypeOfFile::Dir && current_depth < recursive_limit {
            let recursive_width = get_max_width(
                &get_elements_from_path(e.get_path_string(), true),
                recursive_limit,
                current_depth + 1,
            );

            if recursive_width > length {
                length = recursive_width;
            }
        }

        if length > name_max_len {
            name_max_len = length;
        }
    }
    name_max_len
}
