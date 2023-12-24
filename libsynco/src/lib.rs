#![allow(dead_code)]
use notify::{Event, EventHandler, RecommendedWatcher, RecursiveMode, Watcher};
use std::{collections::HashSet, path::Path};
use thiserror::Error as ErrorTrait;

#[derive(Debug, ErrorTrait)]
pub enum Error {
    #[error("Failed to watch directory")]
    WatchError(#[from] notify::Error),
}

type Result<T> = std::result::Result<T, Error>;

pub struct SyncManager {
    watcher: RecommendedWatcher,
    watched_dirs: HashSet<Directory>,
}

impl SyncManager {
    pub fn new() -> Result<Self> {
        let watcher = notify::recommended_watcher(|res| match res {
            Err(e) => eprintln!("{:?}", e),
            Ok(ev) => {}
        })?;
        todo!()
    }

    pub fn watch_dir(&mut self, d: Directory) -> Result<()> {
        if !self.watched_dirs.contains(&d) {
            if d.recursive {
                self.watcher
                    .watch(Path::new(&d.path), RecursiveMode::Recursive)?;
            } else {
                self.watcher
                    .watch(Path::new(&d.path), RecursiveMode::NonRecursive)?;
            }
            self.watched_dirs.insert(d);
        }
        Ok(())
    }
}

pub struct Connection;

#[derive(PartialEq, Eq, Hash)]
pub struct Directory {
    path: String,
    recursive: bool,
}

pub struct ChangeLog {}
