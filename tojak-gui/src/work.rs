use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Event {
    pub name: String,
}

impl Event {
    pub const SUB_DIRECTORY: &str = "events";

    pub fn new(base: &Path, name: String) -> Result<Self, Box<dyn std::error::Error>> {
        let this = Self { name };
        this.write(base)?;
        Ok(this)
    }

    pub fn read(base: &Path, name: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let path = Self::make_path(base, name);
        let this = serde_json::from_str::<Self>(&std::fs::read_to_string(&path)?)?;
        Ok(this)
    }

    fn make_path(base: &Path, name: &str) -> PathBuf {
        base.join(Self::SUB_DIRECTORY)
            .join(name)
            .with_extension("json")
    }

    pub fn write(&self, base: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let path = Self::make_path(base, &self.name);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let file = File::create(path)?;
        serde_json::to_writer_pretty(file, self)?;
        Ok(())
    }

    pub fn rename(&mut self, base: &Path, new: String) -> std::io::Result<()> {
        std::fs::rename(
            Self::make_path(base, &self.name),
            Self::make_path(base, &new),
        )?;
        self.name = new;

        Ok(())
    }
}
