mod editor;
mod meta;
mod render;

use gpui::{App, AppContext, Context, Entity, EventEmitter, Window};
use std::path::PathBuf;

use crate::{EventBus, acc_err, components::SearchBar, universe::editor::Editor};

pub use crate::universe::meta::UniverseMeta;

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

pub enum UniverseEvent {
    SaveEvent,
    OpenEvent(String),
    CreateEvent(String),
}

pub struct UniverseEventBus;

impl EventEmitter<UniverseEvent> for UniverseEventBus {}

#[derive(Clone, PartialEq, gpui::Action)]
pub struct HandleBackspace;

impl Universe {
    pub fn new(meta: UniverseMeta, window: &mut Window, cx: &mut App) -> Entity<Self> {
        let search_bar = cx.new(|cx| SearchBar::new(window, cx));
        let editor = Editor::new(meta.path.clone(), window, cx);

        let this = cx.new(|_| Self {
            meta,
            search_bar,
            editor,
            right_panel: UniverseRightPanel::Empty,
        });

        cx.bind_keys([gpui::KeyBinding::new("backspace", HandleBackspace, None)]);

        let universe_bus = cx.global::<EventBus>().universe.clone();

        this.update(cx, |_, cx| {
            cx.subscribe_in(
                &universe_bus,
                window,
                |this, _, event, window, cx| match event {
                    UniverseEvent::SaveEvent => {
                        this.editor.update(cx, |this, cx| {
                            if let Err(error) = this.save(cx) {
                                acc_err(error.to_string(), cx);
                            }
                        });
                        cx.notify();
                    }
                    UniverseEvent::OpenEvent(name) => {
                        this.right_panel = UniverseRightPanel::EventEdit;
                        this.editor.update(cx, |this, cx| {
                            if let Err(error) = this.open(name, window, cx) {
                                acc_err(error.to_string(), cx);
                                cx.notify();
                            }
                        });
                        cx.notify();
                    }
                    UniverseEvent::CreateEvent(name) => {
                        this.right_panel = UniverseRightPanel::EventEdit;
                        this.editor.update(cx, |editor, cx| {
                            editor.create(name, window, cx);
                            cx.notify();
                        });
                    }
                },
            )
            .detach();
        });

        this
    }

    pub fn create_universe(&mut self, path: PathBuf, name: String, cx: &mut Context<Self>) {
        let meta = UniverseMeta { path, name };
        if let Err(error) = meta.create_on_disk() {
            acc_err(error.to_string(), cx);
        }
        self.meta = meta;
        self.right_panel = UniverseRightPanel::Empty;
        cx.notify();
    }
}
