mod editor;
mod meta;
mod render;

use gpui::{App, AppContext, Context, Entity, EventEmitter, Window};
use gpui_component::input::InputState;
use std::path::{Path, PathBuf};

use crate::{GlobalEvent, components::SearchBar, universe::meta::UniverseMeta, work::Event};

enum UniverseRightPanel {
    Empty,
    EventEdit,
}

pub struct EventRef {
    title: String,
    path: PathBuf,
}

pub struct Universe {
    meta: Option<UniverseMeta>,
    right_panel: UniverseRightPanel,

    // 左パネル
    search_bar: Entity<SearchBar>,
    // 右パネル：執筆画面
    title_field: Entity<InputState>,
    paragraphs: Vec<Entity<InputState>>,
}

pub enum UniverseEvent {
    OpenEvent(String),
    CreateEvent(String),
}

impl EventEmitter<GlobalEvent> for Universe {}

impl EventEmitter<UniverseEvent> for Universe {}

#[derive(Clone, PartialEq, gpui::Action)]
pub struct HandleBackspace;

impl Universe {
    pub fn new(window: &mut Window, cx: &mut App) -> Entity<Self> {
        let search_bar = cx.new(|cx| SearchBar::new(window, cx));
        let title_field = cx.new(|cx| InputState::new(window, cx));
        let first_paragraph = cx.new(|cx| InputState::new(window, cx).auto_grow(1, usize::MAX));

        let paragraphs = vec![first_paragraph.clone()];

        let this = cx.new(|_| Self {
            meta: None,
            search_bar,
            title_field,
            paragraphs,
            right_panel: UniverseRightPanel::Empty,
        });

        cx.bind_keys([gpui::KeyBinding::new("backspace", HandleBackspace, None)]);

        cx.update_entity(&this, |this_mut, cx| {
            this_mut.subscribe_to_paragraph(&first_paragraph, window, cx);

            cx.subscribe_in(&this, window, |this, _, event, window, cx| match event {
                UniverseEvent::OpenEvent(name) => {
                    let path = this
                        .meta
                        .as_ref()
                        .map(|meta| meta.path.clone())
                        .unwrap()
                        .join(Event::SUB_DIRECTORY)
                        .join(name)
                        .with_extension(".json");
                    this.right_panel = UniverseRightPanel::EventEdit;
                    this.title_field.update(cx, |title, cx| {
                        title.set_value(name.clone(), window, cx);
                        cx.notify();
                    });
                    if let Err(error) = this.open_universe(&path, window, cx) {
                        cx.emit(GlobalEvent::PopupError(error.to_string()));
                    }
                }
                UniverseEvent::CreateEvent(name) => {
                    cx.emit(UniverseEvent::OpenEvent(name.clone()));
                }
            })
            .detach();
        });
        this
    }

    pub fn create_universe(&mut self, path: PathBuf, name: String, cx: &mut Context<Self>) {
        let meta = UniverseMeta { path, name };
        if let Err(error) = meta.create_on_disk() {
            cx.emit(GlobalEvent::PopupError(error.to_string()));
        }
        self.meta = Some(meta);
        cx.notify();
    }

    pub fn open_universe(
        &mut self,
        path: &Path,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let meta = UniverseMeta::read_by_path(&path)?;
        self.meta = Some(meta);
        cx.notify();
        Ok(())
    }
}
