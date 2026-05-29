use gpui::{Div, Pixels, div, prelude::*, px, rgb, rgba};

use crate::{BORDER, DARKER_BG, LIGHTER_BG};

pub type ModDiv<T> = fn(&mut T, Div, window: &mut gpui::Window, cx: &mut Context<T>) -> Div;

pub trait Horizontal2PanelLayout {
    const TEXT_WIDTH: Pixels = px(200.);

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

    fn left_panel() -> ModDiv<Self>
    where
        Self: Sized;

    fn right_panel() -> ModDiv<Self>
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
            .child(Self::left_panel()(
                self,
                Self::left_panel_base(),
                window,
                cx,
            ))
            .child(Self::right_panel()(
                self,
                Self::right_panel_base(),
                window,
                cx,
            ))
    }
}
