use std::path::PathBuf;

use gpui::{App, Context, Entity, Window, prelude::*};

use crate::{EventBus, UniverseEvent, acc_err, emit_universe};

use super::{Editor, HandleBackspace, SearchBar, Universe, UniverseMeta, UniverseRightPanel};

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
                    UniverseEvent::Rename((old, new)) => {
                        if let Err(err) = this.editor.read(cx).rename(&old, &new) {
                            acc_err(err.to_string(), cx);
                        }
                        // リネーム時に変更されたメタデータを含めて最新状態を書き込む
                        emit_universe(UniverseEvent::SaveEvent, cx);
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
