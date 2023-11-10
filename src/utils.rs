use std::fs;
use std::time::SystemTime;

use chrono::offset::Utc;
use chrono::DateTime;

use crate::element::Element;

#[inline]
pub fn get_elements_from_path(path: String, all: bool) -> Vec<Element> {
    fs::read_dir(path)
        .unwrap()
        .map(|e| Element::new(e.unwrap().path().to_str().unwrap()))
        .filter(|element| all || !element.get_name().starts_with('.'))
        .collect()
}

pub fn pad_string(s: String, pad: usize, after: bool) -> String {
    let mut s2 = String::new();
    // println!("{}, {}", s.len(), pad);
    if after {
        s2.push_str(s.as_str());
        for _ in 0..(pad - get_string_length(&s)) {
            s2.push(' ');
        }
    } else {
        for _ in 0..(pad - get_string_length(&s)) {
            s2.push(' ');
        }
        s2.push_str(s.as_str());
    }
    s2
}

// Some characters like   counts for more than one
// character when using .len()
#[inline]
pub fn get_string_length(s: &str) -> usize {
    s.chars().collect::<Vec<char>>().len();
    s.chars().count()
}

pub fn get_size_string(bytes: u64) -> String {
    if bytes == 0 {
        return String::from("0.00 B ");
    }
    let bytes_f32 = bytes as f32;
    let exp = bytes_f32.log(1024.0).floor();
    // print!("\n{}", exp);
    let divided_num = bytes_f32 / 1024.0_f32.powf(exp);
    let unit = ['B', 'K', 'M', 'G', 'T', 'P', 'Y', 'E'][exp as usize];
    format!("{:.2} {} ", divided_num, unit)
}

pub fn system_time_to_string(system_time: SystemTime) -> String {
    let datetime: DateTime<Utc> = system_time.into();
    datetime.format("%d-%m-%y %H:%M").to_string()
}

pub fn get_icon_file_type<'a>(filename: String) -> &'a str {
    let extension = filename.split('.').collect::<Vec<&str>>()[1..].join(".");
    match extension.to_lowercase().as_str() {
        "zip" | "rar" | "7zip" | "tar" | "tar.gz" | "tgz" => "󰗄 ",
        "rs" => " ",
        "jpg" | "jpeg" | "png" | "bmp" | "gif" | "webp" | "svg" => "󰋩 ",
        "flv" | "avi" | "mp4" | "webm" | "mov" => " ",
        "exe" | "ini" | "bat" => " ",
        "py" => " ",
        "c" => " ",
        "cpp" => " ",
        "json" => " ",
        "pdf" => "󰈦 ",
        "java" | "jar" => " ",
        "js" => " ",
        "html" => " ",
        "css" => " ",
        "csv" => " ",
        _ => "󰈔 ",
    }
}
