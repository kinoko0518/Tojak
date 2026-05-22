use gpui::{Div, Length, div, prelude::*, px, rgb, rgba};

use crate::components::{BORDER, GHOST_TEXT, LIGHTER_BG};

pub fn tj_island() -> Div {
    div()
        .flex()
        .flex_col()
        .gap_2()
        .bg(rgb(LIGHTER_BG))
        .p_4()
        .rounded(px(10.))
        .border_1()
        .border_color(rgba(BORDER))
}

pub fn tj_border() -> impl IntoElement {
    div().bg(rgba(BORDER)).w_full().h(px(1.))
}

pub fn subtext(text: impl IntoElement) -> Div {
    div()
        .child(text)
        .text_size(px(12.))
        .text_color(rgba(GHOST_TEXT))
}

pub fn description_and_content() -> Div {
    div()
        .w_full()
        .flex()
        .flex_row()
        .justify_between()
        .items_center()
        .gap_16()
}

pub fn title_and_detail(
    width: impl Clone + Into<Length>,
    title: impl IntoElement,
    detail: impl IntoElement,
) -> Div {
    div().w(width).child(title).child(subtext(detail))
}
