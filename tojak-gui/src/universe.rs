mod render;

use std::{fs::read_to_string, path::PathBuf};

use gpui::{App, AppContext, Context, Entity, EventEmitter};
use serde::{Deserialize, Serialize};

use crate::{components::SearchBar, work::Event};

#[derive(Serialize, Deserialize)]
pub struct UniverseMeta {
    pub name: String,
}

enum UniverseRightPanel {
    Empty,
    EventEdit(Event),
}

pub struct Universe {
    meta: UniverseMeta,
    path: PathBuf,
    search_bar: Option<Entity<SearchBar>>,
    right_panel: UniverseRightPanel,
}

const META_TOML_NAME: &str = "universe.toml";

pub struct EventRef {
    title: String,
    path: PathBuf,
}

impl Universe {
    pub fn create_new_event(&mut self, name: String, cx: &mut Context<Self>) {
        match Event::new(&self.path, name) {
            Ok(event) => {
                self.right_panel = UniverseRightPanel::EventEdit(event);
                cx.notify();
            }
            Err(err) => {
                eprintln!("[Tojak] イベントの書き込みに失敗しました：{}", err);
            }
        }
    }

    pub fn change_event_now_on(&mut self, name: String, cx: &mut Context<Self>) {
        match Event::read(&self.path, &name) {
            Ok(event) => {
                self.right_panel = UniverseRightPanel::EventEdit(event);
                cx.notify();
            }
            Err(err) => {
                eprintln!("[Tojak] イベントの読み込みに失敗しました：{}", err);
            }
        }
    }

    pub fn new(
        path: PathBuf,
        name: String,
        cx: &mut App,
    ) -> Result<Entity<Self>, Box<dyn std::error::Error>> {
        let meta = UniverseMeta { name };
        std::fs::create_dir_all(&path)?;
        std::fs::write(&path.join(META_TOML_NAME), toml::to_string_pretty(&meta)?)?;

        let this = cx.new(|_| Self {
            path,
            meta,
            search_bar: None,
            right_panel: UniverseRightPanel::Empty,
        });

        Ok(this)
    }

    pub fn from_path(path: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let meta_toml = read_to_string(path.join(META_TOML_NAME))?;
        let meta = toml::from_str::<UniverseMeta>(&meta_toml)?;
        Ok(Self {
            path,
            meta,
            search_bar: None,
            right_panel: UniverseRightPanel::Empty,
        })
    }

    pub fn get_events(&self, cx: &mut Context<Self>) -> Result<Vec<EventRef>, std::io::Error> {
        Ok(std::fs::read_dir(&self.path.join(Event::SUB_DIRECTORY))?
            .filter_map(|entry| entry.ok())
            .filter_map(|entry| {
                entry
                    .file_name()
                    .to_str()
                    .and_then(|name| {
                        name.rsplit_once(".")
                            .and_then(|(left, right)| ["json"].contains(&right).then(|| left))
                    })
                    .filter(|name: &&str| {
                        self.search_bar
                            .as_ref()
                            .map(|sb| {
                                let text = sb.read(cx).get_text(cx);
                                text.is_empty() || name.contains(&text)
                            })
                            .unwrap_or(true)
                    })
                    .map(|name| EventRef {
                        title: name.into(),
                        path: entry.path(),
                    })
            })
            .collect())
    }
}
