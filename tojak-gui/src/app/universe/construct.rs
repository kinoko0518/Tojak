use std::path::PathBuf;

use gpui::{App, Context, Entity, Window, prelude::*};

use crate::{EventBus, UniverseEvent, acc_err, emit_universe};

use super::{Editor, HandleBackspace, SearchBar, Universe, UniverseMeta, UniverseRightPanel};

impl Universe {
    pub fn new(meta: UniverseMeta, window: &mut Window, cx: &mut App) -> Entity<Self> {
        let search_bar = cx.new(|cx| SearchBar::new(window, cx));
        let editor = Editor::new(meta.path.clone(), window, cx);
        let rename_input = cx.new(|cx| gpui_component::input::InputState::new(window, cx));

        let this = cx.new(|_| Self {
            meta,
            search_bar,
            episode_one: None,
            editor,
            right_panel: UniverseRightPanel::Empty,
            context_menu: None,
            renaming: None,
            rename_input: rename_input.clone(),
        });

        this.update(cx, |_, cx| {
            cx.subscribe_in(&rename_input, window, |this, _, event, window, cx| {
                if let gpui_component::input::InputEvent::PressEnter { .. } = event {
                    if let Some((_, old_name)) = this.renaming.take() {
                        let new_name = this.rename_input.read(cx).text().to_string();
                        emit_universe(UniverseEvent::Rename((old_name, new_name)), cx);
                        window.blur();
                    }
                } else if let gpui_component::input::InputEvent::Blur = event {
                    this.renaming = None;
                    cx.notify();
                }
            })
            .detach();
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
                            // リネーム時に変更されたメタデータを含めて最新状態を書き込む
                            emit_universe(UniverseEvent::SaveEvent, cx);
                        }
                    }
                    UniverseEvent::SetEpisodeOne(title) => {
                        if let Ok(events) = this.meta.read_events(None) {
                            if let Some(event) = events.into_iter().find(|e| e.title == *title) {
                                this.episode_one = Some(event);
                                cx.notify();
                            }
                        }
                    }
                    UniverseEvent::DeleteEvent(title) => {
                        let event = this
                            .meta
                            .read_events(None)
                            .ok()
                            .and_then(|events| events.into_iter().find(|e| e.title == *title));
                        if let Some(event) = event {
                            // ディスク上のファイルを削除
                            if let Err(e) = std::fs::remove_file(&event.path) {
                                acc_err(e.to_string(), cx);
                            }
                            // 今削除されたイベントを開いているなら閉じる
                            if this.editor.clone().map(|editor| {
                                let read = editor.read(cx);
                                let editing = read.editing(cx).name;
                                editing.as_str() == event.title.as_str()
                            }) {
                                this.right_panel = UniverseRightPanel::Empty;
                            }
                            cx.notify();
                        }
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
