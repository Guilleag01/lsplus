use crate::element::Element;
use crate::utils::{get_string_length, pad_string};

pub fn default(mut elements: Vec<Element>) {
    elements.sort_unstable_by_key(|a: &Element| a.get_name());

    let width = term_size::dimensions().unwrap().0;

    let mut i = 1;

    let mut num_columns = 1;
    let mut column_widths: Vec<usize> = Vec::new();

    let mut total = std::usize::MAX;
    while total >= width - 1 {
        total = 0;
        num_columns = (elements.len() as f32 / i as f32).ceil() as usize;
        column_widths = vec![0; num_columns];
        for j in 0..num_columns {
            let mut max_len = 0;
            for k in 0..i {
                if i * j + k < elements.len() {
                    let space = get_string_length(&elements[i * j + k].to_string());
                    if space > max_len {
                        max_len = space;
                    }
                }
            }
            // println!("Max len {}", max_len);
            total += max_len;
            column_widths[j] = max_len;
        }
        i += 1;
    }

    i -= 1;

    for k in 0..i {
        for j in 0..num_columns {
            if i * j + k < elements.len() {
                print!(
                    "{}",
                    pad_string(elements[i * j + k].to_string(), column_widths[j], true)
                );
            }
        }
        println!();
    }
    println!();
}
