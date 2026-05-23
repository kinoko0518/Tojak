use std::{fs::read_to_string, path::PathBuf};

use gpui::{AppContext, Context, Entity, ParentElement, Render, Styled};
use serde::{Deserialize, Serialize};

use crate::components::{Horizontal2PanelLayout, SearchBar, title_and_detail};

#[derive(Serialize, Deserialize)]
pub struct UniverseMeta {
    pub name: String,
}

pub struct Universe {
    meta: UniverseMeta,
    path: PathBuf,
    search_bar: Option<Entity<SearchBar>>,
}

const META_TOML_NAME: &str = "universe.toml";

pub struct EventRef {
    title: String,
    path: PathBuf,
}

impl Universe {
    pub fn new(path: PathBuf, name: String) -> Result<Self, Box<dyn std::error::Error>> {
        let meta = UniverseMeta { name };
        std::fs::create_dir_all(&path)?;
        std::fs::write(&path.join(META_TOML_NAME), toml::to_string_pretty(&meta)?)?;
        Ok(Self {
            path,
            meta,
            search_bar: None,
        })
    }

    pub fn from_path(path: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let meta_toml = read_to_string(path.join(META_TOML_NAME))?;
        let meta = toml::from_str::<UniverseMeta>(&meta_toml)?;
        Ok(Self {
            path,
            meta,
            search_bar: None,
        })
    }

    pub fn get_events(&self, cx: &mut Context<Self>) -> Result<Vec<EventRef>, std::io::Error> {
        Ok(std::fs::read_dir(&self.path)?
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
                                let text = sb.update(cx, |_, cx| sb.read(cx).get_text(cx));
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

impl Horizontal2PanelLayout for Universe {
    fn left_panel() -> crate::components::ModDiv<Self>
    where
        Self: Sized,
    {
        |me, original, window, cx| {
            if me.search_bar.is_none() {
                me.search_bar = Some(cx.new(|cx| SearchBar::new("search-bar", window, cx)));
            }
            let search_bar = me.search_bar.clone().unwrap();

            original.px_4().py_2().child(search_bar).children(
                me.get_events(cx)
                    .unwrap_or(vec![])
                    .into_iter()
                    .map(|event_ref| {
                        title_and_detail(
                            Self::TEXT_WIDTH,
                            event_ref.title,
                            event_ref.path.to_string_lossy().to_string(),
                        )
                        .px_4()
                        .py_2()
                    }),
            )
        }
    }

    fn right_panel() -> crate::components::ModDiv<Self>
    where
        Self: Sized,
    {
        |me, div, _, _| div
    }
}

impl Render for Universe {
    fn render(
        &mut self,
        window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        self.render_layout(window, cx)
    }
}
