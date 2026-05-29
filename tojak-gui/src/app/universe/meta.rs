use crate::{EVENT_EXTENSION, EVENT_SUBDIR, META_TOML_NAME, app::universe::EventRef};
use serde::{Deserialize, Serialize};
use std::{
    fs::read_to_string,
    path::{Path, PathBuf},
};

#[derive(Serialize, Deserialize)]
pub struct UniverseMeta {
    pub path: PathBuf,
    pub name: String,
}

impl UniverseMeta {
    pub fn read_events(&self, query: Option<&str>) -> std::io::Result<Vec<EventRef>> {
        Ok(std::fs::read_dir(&self.path.join(EVENT_SUBDIR))?
            .filter_map(|entry| entry.ok())
            .filter_map(|entry| {
                entry
                    .file_name()
                    .to_str()
                    .and_then(|name| {
                        name.rsplit_once(".")
                            .and_then(|(left, right)| (EVENT_EXTENSION == right).then(|| left))
                    })
                    .filter(|name: &&str| query.map(|query| name.contains(query)).unwrap_or(true))
                    .map(|name| EventRef {
                        title: name.into(),
                        path: entry.path(),
                    })
            })
            .collect())
    }

    pub fn create_on_disk(&self) -> Result<(), Box<dyn std::error::Error>> {
        std::fs::create_dir_all(&self.path)?;
        std::fs::write(
            &self.path.join(META_TOML_NAME),
            toml::to_string_pretty(self)?,
        )?;
        Ok(())
    }

    pub fn read_by_path(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let meta_toml = read_to_string(path.join(META_TOML_NAME))?;
        let meta = toml::from_str::<Self>(&meta_toml)?;
        Ok(meta)
    }
}
