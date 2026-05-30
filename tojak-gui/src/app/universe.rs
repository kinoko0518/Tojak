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

#[derive(Clone, PartialEq)]
pub struct EventRef {
    pub title: String,
    pub path: PathBuf,
}

pub struct Universe {
    meta: UniverseMeta,
    right_panel: UniverseRightPanel,

    episode_one: Option<EventRef>,

    search_bar: Entity<SearchBar>,
    editor: Entity<Editor>,

    pub context_menu: Option<(usize, EventRef, gpui::Point<gpui::Pixels>)>,
    pub renaming: Option<(usize, String)>,
    pub rename_input: Entity<gpui_component::input::InputState>,
}

#[derive(Clone, PartialEq, gpui::Action)]
pub struct HandleBackspace;
