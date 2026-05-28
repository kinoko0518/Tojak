use gpui::{
    AnyElement, Div, IntoElement, ParentElement, Styled, div, prelude::FluentBuilder, px, rgb, rgba,
};

use crate::{
    components::ACCENT,
    universe::editor::command::{ConditonedTransition, ParagraphKind},
};

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
                        cl().child("いずれでもないとき")
                            .child(pill(path))
                            .child("に続く"),
                    )
                }),
        )
}

pub fn render_paragraph(paragrakind: &ParagraphKind) -> AnyElement {
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
