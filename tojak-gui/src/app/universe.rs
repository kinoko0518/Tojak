mod construct;
mod editor;
mod meta;
mod render;

use std::path::PathBuf;

use editor::Editor;
use gpui::Entity;

use crate::components::SearchBar;

pub(crate) use meta::UniverseMeta;

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
