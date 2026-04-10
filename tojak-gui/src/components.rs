use gpui::{App, Div, ElementId, div, prelude::*, px, rgb, rgba};
use gpui_component::{
    Sizable,
    button::{Button, ButtonCustomVariant, ButtonVariants},
};

pub const LIGHTER_BG: u32 = 0x202020;
pub const DARKER_BG: u32 = 0x000000;

pub const BORDER: u32 = 0xffffff20;
pub const GHOST_TEXT: u32 = 0xffffff40;

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

pub fn tj_button(cx: &mut App, id: impl Into<ElementId>) -> Button {
    let variant = ButtonCustomVariant::new(cx)
        .color(rgb(0xffffff).into())
        .hover(rgb(0xc0c0c0).into());
    Button::new(id)
        .custom(variant)
        .rounded(px(f32::MAX))
        .small()
        .p_4()
        .text_color(rgb(0x000000))
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
