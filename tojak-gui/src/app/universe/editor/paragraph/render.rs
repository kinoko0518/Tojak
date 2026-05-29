use gpui::{
    Context, Focusable, InteractiveElement, IntoElement, ParentElement, Render,
    StatefulInteractiveElement, Styled, Window, div, prelude::FluentBuilder, rgb, rgba,
};
use gpui_component::input::Input;

use crate::{
    ACCENT, BORDER, LIGHTER_BG, UniverseEvent,
    app::universe::{
        HandleBackspace,
        editor::{
            Paragraphs, command::parse_paragraph_kind, paragraph::paragraph_disp::render_paragraph,
        },
    },
    emit_universe,
};

impl Render for Paragraphs {
    fn render(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl gpui::prelude::IntoElement {
        div().children(self.inputs.iter().enumerate().map(|(i, paragraph)| {
            let state = paragraph.read(cx);
            let focus_handle = state.focus_handle(cx);

            let target_entity = paragraph.clone();

            div()
                .p_2()
                .child({
                    let text = state.text().as_str().unwrap_or("");
                    let paragraph_kind = parse_paragraph_kind(text);
                    let is_command = text.starts_with(":") || text.starts_with(">");
                    let is_focused = focus_handle.is_focused(window);

                    let command_bg = rgba(ACCENT << 8 | 0x20);
                    let command_border = rgba(ACCENT << 8 | 0x50);

                    let input = Input::new(paragraph)
                        .appearance(false)
                        .p_4()
                        .rounded_lg()
                        .border_1()
                        .when_else(
                            is_command,
                            |this| this.bg(command_bg).border_color(command_border),
                            |this| this.bg(rgb(LIGHTER_BG)).border_color(rgba(BORDER)),
                        )
                        .into_any_element();

                    if is_focused {
                        input
                    } else if is_command {
                        let focus_target = paragraph.clone();

                        div()
                            .p_4()
                            .bg(command_bg)
                            .border_1()
                            .border_color(command_border)
                            .rounded_lg()
                            .id(("command", i))
                            .on_click(cx.listener(move |_this, _event, window, cx| {
                                focus_target.update(cx, |input, cx| {
                                    input.focus(window, cx);
                                });
                                cx.notify();
                            }))
                            .child(render_paragraph(&paragraph_kind))
                            .into_any_element()
                    } else {
                        input
                    }
                })
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
                .id(("paragraph", i))
                .track_focus(&self.focus_handle)
                .on_mouse_down_out(cx.listener(|_, _, window, cx| {
                    emit_universe(UniverseEvent::SaveEvent, cx);
                    window.blur();
                }))
        }))
    }
}
