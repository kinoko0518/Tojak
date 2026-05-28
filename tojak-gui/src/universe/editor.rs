mod command;
mod paragraph;
mod title_edit;

use std::{fs::File, io::Write, path::PathBuf};

use gpui::{
    App, AppContext, Context, Entity, InteractiveElement, ParentElement, Render, Styled, Window,
    div, linear_color_stop, linear_gradient, rgba,
};
use gpui_component::{input::InputState, scroll::ScrollableElement};
pub use paragraph::Paragraphs;
use serde::{Deserialize, Serialize};
pub use title_edit::TitleEdit;

use crate::EVENT_SUBDIR;

pub struct Editor {
    pub base: PathBuf,
    title_field: Entity<TitleEdit>,
    paragraphs: Entity<Paragraphs>,
}

const EVENT_EXTENSION: &str = "json";

#[derive(Serialize, Deserialize)]
pub struct Event {
    pub title: String,
    pub paragraphs: Vec<String>,
}

impl Editor {
    pub fn new(base: PathBuf, window: &mut Window, cx: &mut App) -> Entity<Self> {
        let title_field = TitleEdit::new(window, cx);
        let paragraphs = Paragraphs::empty(window, cx);

        let me = cx.new(|_| Self {
            base,
            title_field,
            paragraphs,
        });

        me
    }

    pub fn create(&mut self, name: &str, window: &mut Window, cx: &mut Context<Self>) {
        let paragraphs = Paragraphs::empty(window, cx);

        self.title_field.update(cx, |this, cx| {
            this.set_field(name.into(), window, cx);
        });
        self.paragraphs = paragraphs;
    }

    pub fn save(&self, cx: &mut App) -> Result<(), Box<dyn std::error::Error>> {
        let title = self.title_field.read(cx).input.read(cx).text().to_string();
        let paragraphs = self
            .paragraphs
            .read(cx)
            .inputs
            .iter()
            .map(|input| input.read(cx).text().to_string())
            .collect();
        let event_dir = self.base.join(EVENT_SUBDIR);
        std::fs::create_dir_all(&event_dir)?;
        let path = event_dir.join(&title).with_extension(EVENT_EXTENSION);

        let event = Event { title, paragraphs };
        let serialized = serde_json::to_string_pretty(&event)?;

        let mut file = File::create(&path)?;

        file.write_all(serialized.as_bytes())?;

        Ok(())
    }

    pub fn open(
        &mut self,
        name: &str,
        window: &mut Window,
        cx: &mut App,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let path = self
            .base
            .join(EVENT_SUBDIR)
            .join(name)
            .with_extension(EVENT_EXTENSION);
        let read = std::fs::read_to_string(path)?;
        let parsed = serde_json::from_str::<Event>(&read)?;

        self.title_field.update(cx, |this, cx| {
            this.set_field(parsed.title.into(), window, cx);
        });

        self.paragraphs = cx.new(|cx| {
            let paragraphs: Vec<Entity<InputState>> = parsed
                .paragraphs
                .iter()
                .map(|text| {
                    cx.new(|cx| {
                        InputState::new(window, cx)
                            .default_value(text.to_string())
                            .auto_grow(1, usize::MAX)
                    })
                })
                .collect();
            let mut paragraph = Paragraphs::new(paragraphs.clone(), cx);
            for p in &paragraphs {
                paragraph.subscribe_to_paragraph(p, window, cx);
            }
            paragraph
        });

        Ok(())
    }

    pub fn rename(&self, old: &str, new: &str) -> std::io::Result<()> {
        let event_dir = self.base.join(EVENT_SUBDIR);
        std::fs::rename(
            event_dir.join(old).with_extension(EVENT_EXTENSION),
            event_dir.join(new).with_added_extension(EVENT_EXTENSION),
        )
    }
}

impl Render for Editor {
    fn render(
        &mut self,
        _: &mut Window,
        _: &mut gpui::prelude::Context<Self>,
    ) -> impl gpui::prelude::IntoElement {
        let contents = div()
            .id("edit-main-scroll")
            .size_full()
            .overflow_y_scrollbar()
            .pt_16()
            .p_8()
            .flex()
            .flex_col()
            .gap_4()
            .child(self.paragraphs.clone());
        let header = div()
            .absolute()
            .top_0()
            .left_0()
            .w_full()
            .h_16()
            .flex()
            .items_center()
            .px_4()
            .bg(linear_gradient(
                180.0,
                linear_color_stop(rgba(0x000000ff), 0.),
                linear_color_stop(rgba(0x00000000), 1.),
            ))
            .child(self.title_field.clone());
        div().size_full().relative().child(contents).child(header)
    }
}
