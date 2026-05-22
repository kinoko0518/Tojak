use gpui::{App, Div, ElementId, Length, div, prelude::*, px, rgb, rgba};
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

pub fn tj_btn(cx: &mut App, id: impl Into<ElementId>) -> Button {
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

pub fn tj_scnd_btn(cx: &mut App, id: impl Into<ElementId>) -> Button {
    let variant = ButtonCustomVariant::new(cx).border(rgba(0xffffff80).into());
    Button::new(id)
        .custom(variant)
        .rounded(px(f32::MAX))
        .border_1()
        .small()
        .p_4()
        .text_color(rgb(0xffffff))
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

pub type ModDiv<T> = fn(&mut T, Div, window: &mut gpui::Window, cx: &mut Context<T>) -> Div;

pub trait Horizontal2PanelLayout {
    fn left_panel_base() -> Div {
        div()
            .w_64()
            .h_full()
            .bg(rgb(LIGHTER_BG))
            .border_r_1()
            .border_color(rgba(BORDER))
    }

    fn right_panel_base() -> Div {
        div()
            .w_full()
            .h_full()
            .bg(rgb(DARKER_BG))
            .flex()
            .flex_col()
            .justify_center()
            .items_center()
    }

    fn left_panel(window: &mut gpui::Window, cx: &mut Context<Self>) -> ModDiv<Self>
    where
        Self: Sized;

    fn right_panel(window: &mut gpui::Window, cx: &mut Context<Self>) -> ModDiv<Self>
    where
        Self: Sized;

    fn render_layout(&mut self, window: &mut gpui::Window, cx: &mut Context<Self>) -> Div
    where
        Self: Sized,
    {
        div()
            .size_full()
            .flex()
            .flex_row()
            .child(Self::left_panel(window, cx)(
                self,
                Self::left_panel_base(),
                window,
                cx,
            ))
            .child(Self::right_panel(window, cx)(
                self,
                Self::right_panel_base(),
                window,
                cx,
            ))
    }
}
