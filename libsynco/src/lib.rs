use std::fs;
use std::path;

#[derive(Debug)]
pub struct File {
    id: String,
    alias: Option<String>,
    name: String,
    size: usize,
}

#[derive(Debug)]
pub struct Directory {
    id: String,
    alias: Option<String>,
    name: String,
    size: usize,
    files: Vec<File>,
}

impl File {
    pub fn new(id: String, alias: Option<String>, name: String, size: usize) -> Self {
        Self {
            id,
            alias,
            name,
            size,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn alias(&self) -> &Option<String> {
        &self.alias
    }
}

impl Directory {
    pub fn new(id: String, alias: Option<String>, name: String, size: usize) -> Self {
        Self {
            id,
            alias,
            name,
            size,
            files: Vec::new(),
        }
    }

    pub fn add_file(&mut self, file: File) {
        self.files.push(file);
    }

    pub fn add_files(&mut self, files: Vec<File>) {
        self.files.extend(files)
    }
}

fn parse_dir(path_: path::Path) -> Directory {
    path_.is_dir();
    todo!
}
