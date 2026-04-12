use crate::components::{description_and_content, title_and_detail, tj_border, tj_btn};
use crate::welcome::{CreateNewWorld, WelcomeScreen, WelcomeState};
use gpui::{Div, div, prelude::*};
use gpui_component::input::InputState;

impl WelcomeScreen {
    pub fn render_main_menu(cx: &mut Context<Self>) -> Div {
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
                        view.state = WelcomeState::CreateNewWorld(CreateNewWorld {
                            name_input,
                            picked_path: None,
                        });
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
}
