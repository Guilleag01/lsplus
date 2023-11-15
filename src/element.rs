use std::{
    fmt::Display,
    fs::{self, read_link},
    os::unix::prelude::FileTypeExt,
    path::Path,
    time::SystemTime,
};

use crate::utils::get_icon_file_type;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TypeOfFile {
    File,
    Dir,
    Link,
    Block,
    Socket,
}

#[derive(Clone)]
pub struct Element {
    path: String,
    file_type: TypeOfFile,
    name: String,
    perms: Option<fs::Permissions>,
    size: u64,
    creation: SystemTime,
}

impl Element {
    pub fn new(path_str: &str) -> Element {
        let path_built = Path::new(path_str);
        let metadata: fs::Metadata;
        if let Result::Ok(m) = fs::metadata(path_str) {
            metadata = m;
        } else {
            return Self {
                path: path_str.to_string(),
                file_type: TypeOfFile::File,
                name: path_str.split('/').last().unwrap().to_string(),
                perms: None,
                size: 0,
                creation: SystemTime::UNIX_EPOCH,
            };
        }
        let symlink_metadata = fs::symlink_metadata(path_str).unwrap();

        let t: TypeOfFile;

        if symlink_metadata.is_symlink() {
            t = TypeOfFile::Link;
        } else if metadata.is_dir() {
            t = TypeOfFile::Dir;
        } else if metadata.is_file() {
            t = TypeOfFile::File;
        } else if metadata.file_type().is_block_device() {
            t = TypeOfFile::Block;
        } else if metadata.file_type().is_socket() {
            t = TypeOfFile::Socket;
        } else {
            t = TypeOfFile::File;
        }

        let mut name = path_built
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        if symlink_metadata.is_symlink() {
            name.push_str(" -> ");
            name.push_str(read_link(path_built).unwrap().to_str().unwrap())
        }

        Self {
            path: path_str.to_string(),
            file_type: t,
            name,
            perms: Some(metadata.permissions()),
            size: if t == TypeOfFile::Dir {
                0
            } else {
                metadata.len()
            },
            creation: metadata.created().unwrap_or(SystemTime::now()),
        }
    }

    pub fn get_path_string(&self) -> String {
        self.path.clone()
    }

    pub fn get_file_type(&self) -> TypeOfFile {
        self.file_type
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_perms(&self) -> Option<fs::Permissions> {
        self.perms.clone()
    }

    pub fn get_size(&self) -> u64 {
        self.size
    }

    pub fn get_creation(&self) -> SystemTime {
        self.creation
    }
}

impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let file_icon = get_icon_file_type(self.get_name());

        let mut is_dir = false;
        let icon = match self.file_type {
            TypeOfFile::File => file_icon,
            TypeOfFile::Dir => {
                is_dir = true;
                " "
            }
            TypeOfFile::Link => "󱅷 ",
            TypeOfFile::Block => "󰋊 ",
            TypeOfFile::Socket => "󰛳 ",
        };
        write!(
            f,
            "{}{}{} ",
            icon,
            self.name.as_str(),
            if is_dir { "/" } else { "" }
        )
    }
}
