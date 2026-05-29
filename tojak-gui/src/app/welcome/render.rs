use super::{GREATEST_CATHEGORY_TERM, WelcomeScreen, WelcomeState};
use crate::{
    GHOST_TEXT,
    components::{Horizontal2PanelLayout, ModDiv, subtext, tj_island},
};
use const_format::concatcp;
use gpui::{Div, Pixels, div, prelude::*, px, rgb, rgba};

impl Horizontal2PanelLayout for WelcomeScreen {
    fn left_panel() -> ModDiv<Self> {
        |me, base, _, _| {
            base.children(me.worlds.iter().map(|world| {
                div()
                    .child(world.name.clone())
                    .child(subtext(
                        world.path.to_str().unwrap_or("無効なパス").to_string(),
                    ))
                    .pl_4()
                    .pr_4()
                    .pt_2()
                    .pb_2()
            }))
        }
    }

    fn right_panel() -> ModDiv<Self> {
        Self::render_right_panel
    }
}

impl WelcomeScreen {
    pub const MENU_WIDTH: Pixels = px(375.);

    pub const TEXT_WIDTH: Pixels = px(200.);
    pub const BUTTON_WIDTH: Pixels = px(75.);

    fn render_right_panel(
        &mut self,
        base: Div,
        _: &mut gpui::Window,
        cx: &mut Context<Self>,
    ) -> Div {
        let (title, island_inside) = match &mut self.state {
            WelcomeState::MainMenu => ("杜若へようこそ！", Self::render_main_menu(cx)),
            WelcomeState::CreateNewWorld(cnw) => (
                concatcp!(GREATEST_CATHEGORY_TERM, "を作成"),
                Self::render_new_world_menu(cnw, cx),
            ),
        };
        let welcome = div()
            .flex()
            .flex_col()
            .items_center()
            .child(div().child(title).text_size(px(20.)))
            .pb_4();
        base.text_color(rgb(0xffffff))
            .child(welcome)
            .child(tj_island().child(island_inside))
            .child(
                div()
                    .text_size(px(12.))
                    .text_color(rgba(GHOST_TEXT))
                    .child(concat!("杜若 v", env!("CARGO_PKG_VERSION")))
                    .pt_4(),
            )
            .child(
                div()
                    .text_color(rgba(GHOST_TEXT))
                    .text_size(px(11.))
                    .child("Powered by Tojak-Engine and aozora.rs"),
            )
    }
}

impl Render for WelcomeScreen {
    fn render(&mut self, window: &mut gpui::Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.render_layout(window, cx)
    }
}
