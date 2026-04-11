use gpui::{Div, Entity, Pixels, SharedString, Window, div, prelude::*, px, rgb, rgba};
use gpui_component::{
    button::{Button, ButtonVariants},
    input::{Input, InputState},
};

use crate::components::{
    BORDER, DARKER_BG, GHOST_TEXT, LIGHTER_BG, description_and_content, subtext, title_and_detail,
    tj_border, tj_btn, tj_island, tj_scnd_btn,
};

enum WelcomeState {
    MainMenu,
    CreateNewWorld { name_input: Entity<InputState> },
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
    path: SharedString,
}

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
            .children(
                self.worlds
                    .iter()
                    .map(|world| div().child(world.name.clone()).child(world.path.clone())),
            )
    }

    fn render_main_menu(window: &mut Window, cx: &mut Context<Self>) -> Div {
        let new_world = description_and_content()
            .child(title_and_detail(
                Self::TEXT_WIDTH,
                "世界を新規作成する",
                "本編、下書き、設定を保管する最上位のワークスペースを作成します",
            ))
            .child(
                tj_btn(cx, "create_world")
                    .label("作成")
                    .w(Self::BUTTON_WIDTH)
                    .on_click(cx.listener(|view, _event, window, cx| {
                        let name_input = cx.new(|cx| InputState::new(window, cx));
                        view.state = WelcomeState::CreateNewWorld { name_input };
                    })),
            );
        let open_world = description_and_content()
            .child(title_and_detail(
                Self::TEXT_WIDTH,
                "世界としてフォルダを開く",
                "世界として作成されたフォルダを開きます",
            ))
            .child(tj_btn(cx, "open_world").label("開く").w(Self::BUTTON_WIDTH));
        div()
            .flex()
            .flex_col()
            .gap_2()
            .min_w(Self::MENU_WIDTH)
            .child(new_world)
            .child(tj_border())
            .child(open_world)
    }

    fn render_new_world_menu(name_input: &Entity<InputState>, cx: &mut Context<Self>) -> Div {
        div()
            .min_w(Self::MENU_WIDTH)
            .flex()
            .flex_col()
            .justify_start()
            .gap_2()
            .child(
                description_and_content()
                    .child(title_and_detail(
                        Self::TEXT_WIDTH,
                        "世界の名称",
                        "作成する世界の名称を設定します",
                    ))
                    .child(Input::new(name_input)),
            )
            .child(tj_border())
            .child(
                description_and_content()
                    .child(title_and_detail(
                        Self::TEXT_WIDTH,
                        "保存場所を選択",
                        "選択したフォルダの下に世界のデータを保存するフォルダが作成されます",
                    ))
                    .child(
                        tj_btn(cx, "world_save_location")
                            .label("選択")
                            .w(Self::BUTTON_WIDTH),
                    ),
            )
            .child(tj_border())
            .child(
                div()
                    .w_full()
                    .flex()
                    .flex_row()
                    .justify_center()
                    .gap_2()
                    .child(
                        tj_scnd_btn(cx, "back_to_main_menu")
                            .label("戻る")
                            .w(Self::BUTTON_WIDTH)
                            .on_click(cx.listener(|view, _, _, _| {
                                view.state = WelcomeState::MainMenu;
                            })),
                    )
                    .child(
                        tj_btn(cx, "create_world")
                            .label("作成")
                            .w(Self::BUTTON_WIDTH),
                    ),
            )
    }

    fn render_rightpanel(&self, window: &mut Window, cx: &mut Context<Self>) -> Div {
        let (title, island_inside) = match &self.state {
            WelcomeState::MainMenu => ("杜若へようこそ！", Self::render_main_menu(window, cx)),
            WelcomeState::CreateNewWorld { name_input } => {
                ("世界を作成", Self::render_new_world_menu(name_input, cx))
            }
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
                    .child(concat!("杜若 v", env!("CARGO_PKG_VERSION"))),
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
