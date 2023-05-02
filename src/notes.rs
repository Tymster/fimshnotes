use crate::Result;
use std::fs::{create_dir, create_dir_all, read_to_string, File};
use std::io::{Read, Write};
use std::path::PathBuf;
#[derive(Debug)]
pub struct Notes {
    pub base: PathBuf,
    pub path: PathBuf,
    pub note: Option<Note>,
}
#[derive(Debug)]
pub struct Note {
    pub name: String,
    pub content: String,
    pub path: PathBuf,
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
    pub fn save(&self) -> Result<()> {
        if let Some(note) = &self.note {
            File::create(&note.path)?.write(note.content.as_bytes())?;
        }
        Ok(())
    }
    pub fn new_note(&self, name: &str) -> Result<()> {
        if !PathBuf::from(name).exists()
            && match (
                self.path.join(name).canonicalize(),
                self.base.canonicalize(),
            ) {
                (Ok(path), Ok(base)) => path.starts_with(base),
                _ => false,
            }
        {
            File::create(format!("{}/{name}.md", self.path.to_str().unwrap()))?;
            return Ok(());
        }
        Err("Note with that name already exsits".into())
    }
    pub fn enter(&mut self, name: &str) -> Result<()> {
        if !self.path.join(name).exists() {
            return Err("No Folder with that name found".into());
        } else if !match (
            self.path.join(name).canonicalize(),
            self.base.canonicalize(),
        ) {
            (Ok(path), Ok(base)) => path.starts_with(base),
            _ => false,
        } {
            println!("REAL");
            return Err("Cannot go furhter back than base".into());
        } else if name == "" {
            self.path = self.base.clone();
            return Ok(());
        } else {
            self.path.push(name);
            Ok(())
        }
    }
    pub fn new_folder(&self, name: &str) -> Result<()> {
        create_dir(self.path.join(name))?;
        Ok(())
    }
    pub fn open(&mut self, name: &str) -> Result<()> {
        let mut path = self.path.join(name);
        path.set_extension("md");
        self.note = Some(Note {
            name: name.clone().to_string(),
            content: read_to_string(&path)?,
            path: path.clone(),
        });
        Ok(())
    }
}
