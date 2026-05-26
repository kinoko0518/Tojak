use gpui::{
    AppContext, Context, Entity, InteractiveElement, ParentElement, Render, Styled, Window, div,
    rgb, rgba,
};
use gpui_component::input::{Input, InputEvent, InputState};

use crate::{
    components::{BORDER, LIGHTER_BG},
    universe::HandleBackspace,
};

pub struct Paragraphs {
    inputs: Vec<Entity<InputState>>,
}

impl Paragraphs {
    pub fn new(inputs: Vec<Entity<InputState>>) -> Self {
        Paragraphs { inputs }
    }
}

impl Render for Paragraphs {
    fn render(
        &mut self,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl gpui::prelude::IntoElement {
        div().children(self.inputs.iter().map(|paragraph| {
            let target_entity = paragraph.clone();
            div()
                .p_2()
                .child(
                    Input::new(paragraph)
                        .appearance(false)
                        .bg(rgb(LIGHTER_BG))
                        .p_4()
                        .rounded_lg()
                        .border_1()
                        .border_color(rgba(BORDER)),
                )
                .on_action(cx.listener(move |this, _: &HandleBackspace, window, cx| {
                    let is_empty = target_entity
                        .read(cx)
                        .text()
                        .as_str()
                        .map(|text| text.is_empty())
                        .unwrap_or(true);

                    if is_empty {
                        this.remove_empty_paragraph(&target_entity, window, cx);
                    } else {
                        cx.propagate();
                    }
                }))
        }))
    }
}

impl Paragraphs {
    pub fn subscribe_to_paragraph(
        &mut self,
        paragraph: &Entity<InputState>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        cx.subscribe_in(
            paragraph,
            window,
            |this, entity, event, window, cx| match event {
                InputEvent::PressEnter { secondary } => {
                    let _ = secondary;
                    if let Some(text) = entity.read(cx).text().as_str()
                        && text.ends_with("\n\n")
                    {
                        let trimmed_text = text.trim_end_matches("\n\n").to_string();
                        entity.update(cx, |input, cx| {
                            input.set_value(trimmed_text, window, cx);
                        });

                        this.insert_paragraph_after(entity, window, cx);
                    }
                }
                _ => (),
            },
        )
        .detach();
    }

    /// 指定されたパラグラフの直後に新しいパラグラフを挿入し、フォーカスを当てる
    fn insert_paragraph_after(
        &mut self,
        target_entity: &Entity<InputState>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(index) = self.inputs.iter().position(|p| p == target_entity) {
            let new_paragraph = cx.new(|cx| InputState::new(window, cx).auto_grow(1, usize::MAX));
            self.subscribe_to_paragraph(&new_paragraph, window, cx);
            self.inputs.insert(index + 1, new_paragraph.clone());

            new_paragraph.update(cx, |input, cx| {
                input.focus(window, cx);
            });
            cx.notify();
        }
    }

    pub fn remove_empty_paragraph(
        &mut self,
        target_entity: &Entity<InputState>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(index) = self.inputs.iter().position(|p| p == target_entity) {
            if self.inputs.len() <= 1 {
                return;
            }

            let prev_index = index.saturating_sub(1);
            if let Some(prev_paragraph) = self.inputs.get(prev_index) {
                prev_paragraph.update(cx, |input, cx| {
                    input.focus(window, cx);
                });
            }

            self.inputs.remove(index);
            cx.notify();
        }
    }
}
