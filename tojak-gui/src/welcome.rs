use gpui::{App, Div, Pixels, div, prelude::*, px, rgb, rgba};

use crate::{
    Kakitubata,
    components::{
        BORDER, DARKER_BG, GHOST_TEXT, LIGHTER_BG, subtext, tj_border, tj_button, tj_island,
    },
};

impl Kakitubata {
    fn render_leftpanel() -> Div {
        div()
            .w_64()
            .h_full()
            .bg(rgb(LIGHTER_BG))
            .border_r_1()
            .border_color(rgba(BORDER))
    }

    fn render_rightpanel(cx: &mut App) -> Div {
        const TEXT_WIDTH: Pixels = px(200.);
        const BUTTON_WIDTH: Pixels = px(75.);

        let new_world = div()
            .flex()
            .flex_row()
            .items_center()
            .child(
                div()
                    .w(TEXT_WIDTH)
                    .child("世界を新規作成する")
                    .child(subtext(
                        "本編、下書き、設定を保管する最上位のワークスペースを作成します",
                    )),
            )
            .gap_16()
            .child(tj_button(cx, "create_world").label("作成").w(BUTTON_WIDTH));
        let open_world = div()
            .flex()
            .flex_row()
            .items_center()
            .child(
                div()
                    .w(TEXT_WIDTH)
                    .child("世界としてフォルダを開く")
                    .child(subtext("世界として作成されたフォルダを開きます")),
            )
            .gap_16()
            .child(tj_button(cx, "open_world").label("開く").w(BUTTON_WIDTH));
        let welcome = div()
            .flex()
            .flex_col()
            .items_center()
            .child(div().child("杜若へようこそ！").text_size(px(20.)))
            .child(
                div()
                    .child(concat!("v", env!("CARGO_PKG_VERSION")))
                    .text_color(rgba(GHOST_TEXT)),
            );
        div()
            .w_full()
            .h_full()
            .bg(rgb(DARKER_BG))
            .flex()
            .flex_col()
            .justify_center()
            .items_center()
            .text_color(rgb(0xffffff))
            .gap_2()
            .child(welcome)
            .child(
                tj_island()
                    .child(new_world)
                    .child(tj_border())
                    .child(open_world),
            )
            .child(
                div()
                    .text_color(rgba(GHOST_TEXT))
                    .text_size(px(11.))
                    .child("Powered by Tojak-Engine and aozora.rs"),
            )
    }

    pub fn render_welcome(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_row()
            .child(Self::render_leftpanel())
            .child(Self::render_rightpanel(cx))
    }
}
