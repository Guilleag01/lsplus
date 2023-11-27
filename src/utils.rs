use crate::element::Element;
use chrono::offset::Utc;
use chrono::DateTime;
use clap::ValueEnum;
use std::{cmp::Reverse, fmt::Display, fs, time::SystemTime};

#[derive(ValueEnum, Clone, Debug)]
pub enum SortBy {
    NONE,
    NAME,
    SIZE,
    CREATION,
}

impl Display for SortBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Self::NONE => "none",
            Self::NAME => "name",
            Self::SIZE => "size",
            Self::CREATION => "creation",
        };
        write!(f, "{}", text)
    }
}

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
    let s_length = get_string_length(&s);
    if after {
        s2.push_str(s.as_str());
        for _ in 0..(pad.saturating_sub(s_length)) {
            s2.push(' ');
        }
    } else {
        for _ in 0..(pad.saturating_sub(s_length)) {
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
    s.chars().count()
}

pub fn get_size_string(bytes: u64) -> String {
    if bytes == 0 {
        return String::from("0.00 B ");
    }
    let bytes_f32 = bytes as f32;
    let exp = bytes_f32.log(1024.0).floor();
    let divided_num = bytes_f32 / 1024.0_f32.powf(exp);
    let unit = ['B', 'K', 'M', 'G', 'T', 'P', 'Y', 'E'][exp as usize];
    if unit == 'B' {
        format!("{:.0} {} ", divided_num, unit)
    } else {
        format!("{:.2} {} ", divided_num, unit)
    }
}

#[inline]
pub fn system_time_to_string(system_time: SystemTime) -> String {
    let datetime: DateTime<Utc> = system_time.into();
    datetime.format("%d-%m-%y %H:%M").to_string()
}

pub fn sort_elements(elements: &mut [Element], sort_by: &SortBy) {
    match sort_by {
        SortBy::NONE => (),
        SortBy::NAME => elements.sort_unstable_by_key(|a| a.get_name().to_lowercase()),
        SortBy::SIZE => elements.sort_unstable_by_key(|e| Reverse(e.get_size())),
        SortBy::CREATION => elements.sort_unstable_by_key(|e| Reverse(e.get_creation())),
    }
}

// ALL ICONS MUST BE FOLLOWED BY A SPACE
pub fn get_icon_file_type<'a>(filename: String) -> &'a str {
    let extension = filename.split('.').last().unwrap(); //.collect::<Vec<&str>>()[1..].join(".");
    match extension.to_lowercase().as_str() {
        "jpg" | "jpeg" | "png" | "bmp" | "gif" | "webp" | "svg" => "󰋩 ",
        "zip" | "rar" | "7zip" | "tar" | "gz" | "tgz" => "󰗄 ",
        "flv" | "avi" | "mp4" | "webm" | "mov" => " ",
        "config" | "toml" | "yaml" | "conf" => "󰒓 ",
        "exe" | "bat" | "dll" | "msi" => " ",
        "mp3" | "wav" | "ogg" => " ",
        "java" | "jar" => " ",
        "py" | "ipynb" => " ",
        "csv" | "sql" => " ",
        "ttf" | "fnt" => " ",
        "gitignore" => " ",
        "b" | "bf" => "󰧑 ",
        "makefile" => " ",
        "json" => " ",
        "html" => " ",
        "lock" => "󰌾 ",
        "cpp" => " ",
        "pdf" => " ",
        "css" => " ",
        "bin" => " ",
        "asm" => " ",
        "txt" => " ",
        "xml" => "󰗀 ",
        "lua" => " ",
        "php" => " ",
        "vim" => " ",
        "rs" => " ",
        "js" => " ",
        "sh" => " ",
        "db" => "󰆼 ",
        "cs" => " ",
        "c" => " ",
        "r" => " ",
        _ => "󰈔 ",
    }
}
