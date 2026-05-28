use gpui::{
    AnyElement, App, AppContext, Context, Div, Entity, FocusHandle, Focusable, InteractiveElement,
    IntoElement, ParentElement, Render, StatefulInteractiveElement, Styled, Window, div,
    prelude::FluentBuilder, px, rgb, rgba,
};
use gpui_component::input::{Input, InputEvent, InputState};

use crate::{
    components::{ACCENT, BORDER, LIGHTER_BG},
    emit_universe,
    universe::{
        HandleBackspace, UniverseEvent,
        editor::command::{ConditonedTransition, ParagraphKind, parse_paragraph_kind},
    },
};

pub struct Paragraphs {
    pub inputs: Vec<Entity<InputState>>,
    focus_handle: FocusHandle,
}

impl Paragraphs {
    pub fn new(inputs: Vec<Entity<InputState>>, cx: &mut App) -> Self {
        Paragraphs {
            inputs,
            focus_handle: cx.focus_handle(),
        }
    }

    pub fn empty(window: &mut Window, cx: &mut App) -> Entity<Self> {
        let first = cx.new(|cx| InputState::new(window, cx).auto_grow(1, usize::MAX));
        let inputs = vec![first.clone()];

        let this = cx.new(|cx| {
            let mut paragraph = Paragraphs {
                inputs,
                focus_handle: cx.focus_handle(),
            };
            paragraph.subscribe_to_paragraph(&first, window, cx);
            paragraph
        });

        this
    }
}

fn cl() -> Div {
    div().flex().flex_row().gap_0p5().items_center()
}

fn pill(name: impl Into<String>) -> Div {
    let flashy_cc = ACCENT << 8 | 0x50;

    div()
        .bg(rgba(flashy_cc))
        .px_2()
        .rounded_full()
        .text_size(px(11.))
        .text_color(rgb(ACCENT))
        .child(name.into())
}

fn bino(left: &str, between: &'static str, right: &str, after: &'static str) -> Div {
    cl().child(pill(left))
        .child(between)
        .child(pill(right))
        .child(after)
}

fn uno(name: &str, after: &'static str) -> Div {
    cl().child(pill(name)).child(after)
}

fn conditioned_transition(ct: &ConditonedTransition) -> Div {
    div()
        .flex()
        .flex_row()
        .items_start()
        .child(
            cl().children(ct.condition.iter().copied().map(|(left, right)| {
                cl().child(pill(left))
                    .child("が")
                    .child(pill(right))
                    .child("を")
            })),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .children(ct.branch.iter().map(|(cond, path)| {
                    cl().children(
                        cond.iter()
                            .map(|b| pill(if *b { "保有" } else { "非保有" })),
                    )
                    .child("のとき")
                    .child(pill(*path))
                    .child("に続く")
                }))
                .when_some(ct.wildcase, |this, path| {
                    this.child(
                        cl().child("いずれでもなければ")
                            .child(pill(path))
                            .child("に続く"),
                    )
                }),
        )
}

fn render_paragraph(paragrakind: &ParagraphKind) -> AnyElement {
    match paragrakind {
        ParagraphKind::Get((left, right)) => bino(left, "が", right, "を獲得").into_any_element(),
        ParagraphKind::Lost((left, right)) => bino(left, "が", right, "を喪失").into_any_element(),
        ParagraphKind::Both((left, right)) => {
            bino(left, "が", right, "を獲得・喪失しうる").into_any_element()
        }
        ParagraphKind::Nether((left, right)) => {
            bino(left, "と", right, "の関係性を切断する").into_any_element()
        }
        ParagraphKind::AssertSome((left, right)) => {
            bino(left, "は", right, "を持っている").into_any_element()
        }
        ParagraphKind::AssertSomative((left, right)) => {
            bino(left, "は", right, "を持ちうる").into_any_element()
        }
        ParagraphKind::AssertNone((left, right)) => {
            bino(left, "は", right, "を持っていない").into_any_element()
        }
        ParagraphKind::AssertNonative((left, right)) => {
            bino(left, "は", right, "を持っていない可能性がある").into_any_element()
        }
        ParagraphKind::UnconditionedTransition(path) => uno(path, "に続く").into_any_element(),
        ParagraphKind::ConditonedTransition(ct) => conditioned_transition(&ct).into_any_element(),
        ParagraphKind::Otherwise(t) => div()
            .flex()
            .flex_row()
            .text_color(rgb(ACCENT))
            .child("無効なコマンド：")
            .child(t.to_string())
            .into_any_element(),
    }
}

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
