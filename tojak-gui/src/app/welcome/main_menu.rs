use super::{CreateNewUniverse, WelcomeScreen, WelcomeState};
use crate::components::{description_and_content, title_and_detail, tj_border, tj_btn};
use crate::{GREATEST_CATHEGORY_TERM, GlobalEvent, emit_global};
use const_format::concatcp;
use gpui::{Div, div, prelude::*};
use gpui_component::input::InputState;

impl WelcomeScreen {
    pub fn render_main_menu(cx: &mut Context<Self>) -> Div {
        let new_world = description_and_content()
            .child(title_and_detail(
                Self::TEXT_WIDTH,
                concatcp!(GREATEST_CATHEGORY_TERM, "を新規作成する"),
                "本編、下書き、設定を保管する最上位のワークスペースを作成します",
            ))
            .child(
                tj_btn(cx, "create-universe")
                    .label("作成")
                    .w(Self::BUTTON_WIDTH)
                    .on_click(cx.listener(|view, _event, window, cx| {
                        let name_input = cx.new(|cx| InputState::new(window, cx));
                        view.state = WelcomeState::CreateNewUniverse(CreateNewUniverse {
                            name_input,
                            picked_path: None,
                        });
                    })),
            );
        let select_fn = cx.listener(|_, _, _, cx| {
            cx.spawn(async move |view, cx| {
                let path = rfd::AsyncFileDialog::new()
                    .pick_folder()
                    .await
                    .map(|file| file.path().to_path_buf());
                if let Some(path) = path {
                    let _ = view.update(cx, |_, cx| {
                        emit_global(GlobalEvent::OpenUniverse(path), cx);
                    });
                }
            })
            .detach();
        });
        let open_world = description_and_content()
            .child(title_and_detail(
                Self::TEXT_WIDTH,
                concatcp!(GREATEST_CATHEGORY_TERM, "を開く"),
                concatcp!(GREATEST_CATHEGORY_TERM, "としてフォルダを開きます"),
            ))
            .child(
                tj_btn(cx, "open-universe")
                    .label("開く")
                    .w(Self::BUTTON_WIDTH)
                    .on_click(select_fn),
            );
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
