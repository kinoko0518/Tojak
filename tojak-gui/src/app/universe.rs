mod construct;
mod editor;
mod meta;
mod render;

use std::path::PathBuf;

use editor::Editor;
use gpui::Entity;

use crate::components::SearchBar;

pub(crate) use meta::UniverseMeta;

pub(crate) const META_TOML_NAME: &str = "universe.toml";
pub(crate) const EVENT_SUBDIR: &str = "events";
pub(crate) const EVENT_EXTENSION: &str = "json";

enum UniverseRightPanel {
    Empty,
    EventEdit,
}

pub struct EventRef {
    title: String,
    path: PathBuf,
}

pub struct Universe {
    meta: UniverseMeta,
    right_panel: UniverseRightPanel,

    search_bar: Entity<SearchBar>,
    editor: Entity<Editor>,
}

#[derive(Clone, PartialEq, gpui::Action)]
pub struct HandleBackspace;
