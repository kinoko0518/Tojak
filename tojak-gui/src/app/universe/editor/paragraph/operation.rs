use gpui::{AppContext, Context, Entity, Window};
use gpui_component::input::{InputEvent, InputState};

use crate::{UniverseEvent, app::universe::editor::Paragraphs, emit_universe};

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
                    emit_universe(UniverseEvent::SaveEvent, cx);

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
