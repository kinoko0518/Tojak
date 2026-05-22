use std::{fs::read_to_string, path::PathBuf};

use gpui::{Render, div};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UniverseMeta {
    pub name: String,
}

pub struct Universe {
    path: PathBuf,
    meta: UniverseMeta,
}

const META_TOML_NAME: &str = "universe.toml";

impl Universe {
    pub fn new(path: PathBuf, name: String) -> Result<Self, Box<dyn std::error::Error>> {
        let meta = UniverseMeta { name };
        std::fs::create_dir_all(&path)?;
        std::fs::write(&path.join(META_TOML_NAME), toml::to_string_pretty(&meta)?)?;
        Ok(Self { path, meta })
    }

    pub fn from_path(path: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let meta_toml = read_to_string(path.join(META_TOML_NAME))?;
        let meta = toml::from_str::<UniverseMeta>(&meta_toml)?;
        Ok(Self { path, meta })
    }
}

impl Render for Universe {
    fn render(
        &mut self,
        window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        div()
    }
}
