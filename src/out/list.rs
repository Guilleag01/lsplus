use crate::element::{Element, TypeOfFile};
use crate::utils::{
    get_elements_from_path, get_size_string, get_string_length, pad_string, sort_elements,
    system_time_to_string, SortBy,
};

pub fn list(mut elements: Vec<Element>, recursive_limit: usize, sort_by: SortBy) {
    //  ╭──────────────╼ File name ╾──────────────┬─╼ Size ╾─┬──╼ Creation ╾──╮
    //  │ some_example_file                       │ 420.69 G │ 01-01-70 00:00 │
    //  ╰─────────────────────────────────────────┴──────────┴────────────────╯
    let width = term_size::dimensions().unwrap().0;

    let name_max_len = get_max_width(&elements, recursive_limit, 0);

    let name_length = name_max_len
        .max(14 + (elements.len() as f32).log10() as usize)
        .max(13)
        .min(width - 31);

    print_header(name_length);
    let num_elements = print_elements(
        &mut elements,
        name_length,
        recursive_limit,
        0,
        &Vec::new(),
        &sort_by,
    );
    print_footer(num_elements, name_length);
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
    elements: &mut Vec<Element>,
    name_length: usize,
    recursive_limit: usize,
    current_depth: usize,
    is_last_element: &[bool],
    sort_by: &SortBy,
) -> usize {
    sort_elements(elements, sort_by);

    let mut num_elements = elements.len();

    let mut new_is_last_element = is_last_element.to_owned();

    for (i, e) in elements.iter().enumerate() {
        print!("│ ");
        // text.push_str("│ ");
        let mut e_string = String::new();
        if current_depth > 0 {
            add_recursive_lines(
                &mut e_string,
                &new_is_last_element,
                i == elements.len() - 1,
                "╰─",
                "├─",
            );
        }

        e_string.push_str(e.to_string().as_str());
        let file_text = pad_string(
            get_slice_of_string(e_string.as_str(), name_length - 1, 0, current_depth),
            name_length,
            true,
        );

        print!(
            "{}",
            file_text // if e.get_file_type() == TypeOfFile::Dir {
                      //     file_text.cyan().to_string()
                      // } else {
                      //     file_text
                      // }
        );

        // text.push_str(
        //     pad_string(
        //         get_slice_of_string(e_string.as_str(), name_length - 1, 0, current_depth),
        //         name_length,
        //         true,
        //     )
        //     .as_str(),
        // );

        print_size_and_creation_date(e);

        let num_splits =
            get_string_length(e.get_name().as_str()) / (name_length - (2 + 2 * current_depth));

        for j in 1..num_splits + 1 {
            let mut e_name = String::from("│ ");
            if current_depth > 0 {
                add_recursive_lines(
                    &mut e_name,
                    &new_is_last_element,
                    i == elements.len() - 1,
                    "  ",
                    "│ ",
                )
            }
            e_name.push_str("  ");
            e_name.push_str(
                get_slice_of_string(
                    e.get_name().as_str(),
                    name_length - (3 + 2 * current_depth),
                    j,
                    current_depth,
                )
                .as_str(),
            );
            print!("{}", pad_string(e_name, name_length + 2, true));
            // text.push_str(pad_string(e_name, name_length + 2, true).as_str());
            println!("│          │                │");
            // text.push_str("│          │                │\n");
        }

        if e.get_file_type() == TypeOfFile::Dir && current_depth < recursive_limit {
            let dir_path = e.get_path_string();
            new_is_last_element.push(i == elements.len() - 1);
            num_elements += print_elements(
                &mut get_elements_from_path(dir_path, true),
                name_length,
                recursive_limit,
                current_depth + 1,
                &new_is_last_element,
                sort_by,
            );
            new_is_last_element.pop();
        }
    }
    num_elements
}

fn add_recursive_lines(
    e_string: &mut String,
    is_last_element: &[bool],
    is_last: bool,
    str_is_last: &str,
    str_is_not_last: &str,
) {
    for &is_last in &is_last_element[1..] {
        if is_last {
            e_string.push_str("  ");
        } else {
            e_string.push_str("│ ");
        }
    }

    if is_last {
        e_string.push_str(str_is_last);
    } else {
        e_string.push_str(str_is_not_last);
    }
}

#[inline]
fn get_slice_of_string(e: &str, name_length: usize, i: usize, _current_depth: usize) -> String {
    e.chars().collect::<Vec<char>>()[((name_length) * i).min(get_string_length(e))
        ..((name_length) * (i + 1)).min(get_string_length(e))]
        .iter()
        .collect()
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

fn print_footer(num_elements: usize, name_length: usize) {
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
        let mut length = e.to_string().chars().count() + current_depth * 2;
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
