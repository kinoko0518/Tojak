mod create_new_world;
mod main_menu;

use std::path::PathBuf;

use crate::components::{BORDER, DARKER_BG, GHOST_TEXT, LIGHTER_BG, subtext, tj_island};
pub use crate::welcome::create_new_world::CreateNewWorld;
use const_format::concatcp;
use gpui::{Div, EventEmitter, Pixels, SharedString, Window, div, prelude::*, px, rgb, rgba};

pub enum WelcomeEvent {
    GoToWorkspace(PathBuf),
}

impl EventEmitter<WelcomeEvent> for WelcomeScreen {}

enum WelcomeState {
    MainMenu,
    CreateNewWorld(CreateNewWorld),
}

pub struct WelcomeScreen {
    state: WelcomeState,
    worlds: Vec<WorldRef>,
}

impl Default for WelcomeScreen {
    fn default() -> Self {
        Self {
            state: WelcomeState::MainMenu,
            worlds: Vec::new(),
        }
    }
}

struct WorldRef {
    name: SharedString,
    path: PathBuf,
}

const GREATEST_CATHEGORY_TERM: &str = "ユニバース";

impl WelcomeScreen {
    const MENU_WIDTH: Pixels = px(375.);

    const TEXT_WIDTH: Pixels = px(200.);
    const BUTTON_WIDTH: Pixels = px(75.);

    fn render_leftpanel(&self) -> Div {
        div()
            .w_64()
            .h_full()
            .bg(rgb(LIGHTER_BG))
            .border_r_1()
            .border_color(rgba(BORDER))
            .children(self.worlds.iter().map(|world| {
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

    fn render_rightpanel(&mut self, _: &mut Window, cx: &mut Context<Self>) -> Div {
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

        div()
            .w_full()
            .h_full()
            .bg(rgb(DARKER_BG))
            .flex()
            .flex_col()
            .justify_center()
            .items_center()
            .text_color(rgb(0xffffff))
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
        div()
            .size_full()
            .flex()
            .flex_row()
            .child(Self::render_leftpanel(self))
            .child(Self::render_rightpanel(self, window, cx))
    }
}
