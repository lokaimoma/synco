use std::path;

#[derive(Debug)]
pub struct File {
    id: String,
    alias: Option<String>,
    name: String,
    size: u64,
    path: Option<path::PathBuf>,
}

#[derive(Debug)]
pub struct Directory {
    id: String,
    alias: Option<String>,
    name: String,
    size: u64,
    files: Vec<File>,
    path: path::PathBuf,
}

impl File {
    pub fn new(
        id: String,
        alias: Option<String>,
        name: String,
        size: u64,
        path: Option<path::PathBuf>,
    ) -> Self {
        Self {
            id,
            alias,
            name,
            size,
            path,
        }
    }
}

impl Directory {
    pub fn new(
        id: String,
        alias: Option<String>,
        name: String,
        size: u64,
        path: path::PathBuf,
    ) -> Self {
        Self {
            id,
            alias,
            name,
            size,
            files: Vec::new(),
            path,
        }
    }
}
