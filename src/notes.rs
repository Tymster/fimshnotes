use crate::Result;
use std::fs::{create_dir, create_dir_all, File};
use std::io::{Read, Write};
use std::path::PathBuf;
#[derive(Debug)]
pub struct Notes {
    base: PathBuf,
    path: PathBuf,
    note: Option<Note>,
}
#[derive(Debug)]
struct Note {
    name: String,
    content: String,
    path: PathBuf,
}
impl Notes {
    pub fn new<T: Into<PathBuf> + Clone>(path: T) -> Result<Self> {
        let path: PathBuf = path.into();
        if !PathBuf::from(&path).exists() {
            create_dir_all(&path.parent().unwrap())?;
            create_dir(&path)?;
        }
        Ok(Self {
            base: path.clone().into(),
            path: path.clone().into(),
            note: None,
        })
    }
    pub fn new_note(&self, name: &str) -> Result<()> {
        println!("BALLS in {}", self.path.join(name).display());
        if !PathBuf::from(name).exists() {
            File::create(format!("{}/{name}.md", self.path.to_str().unwrap()))?;
            return Ok(());
        }
        Err("Note with that name already exsits".into())
    }
    pub fn new_folder(&self, name: &str) -> Result<()> {
        create_dir(self.path.join(name))?;
        Ok(())
    }
}
