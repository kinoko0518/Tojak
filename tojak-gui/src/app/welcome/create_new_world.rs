use std::path::PathBuf;

use const_format::concatcp;
use gpui::{Div, Entity, SharedString, div, prelude::*};
use gpui_component::{
    Disableable,
    input::{Input, InputState},
};

use crate::{
    GREATEST_CATHEGORY_TERM, GlobalEvent,
    components::{
        description_and_content, subtext, title_and_detail, tj_border, tj_btn, tj_scnd_btn,
    },
    emit_global,
};

use super::{WelcomeScreen, WelcomeState, WorldRef};

pub struct CreateNewUniverse {
    pub name_input: Entity<InputState>,
    pub picked_path: Option<PathBuf>,
}

impl WelcomeScreen {
    pub fn render_new_world_menu(cnw: &mut CreateNewUniverse, cx: &mut Context<Self>) -> Div {
        let select_fn = cx.listener(|_view, _, _, cx| {
            cx.spawn(async move |view, cx| {
                let path = rfd::AsyncFileDialog::new()
                    .pick_folder()
                    .await
                    .map(|file| file.path().to_path_buf());

                let _ = view.update(cx, |view, cx| {
                    if let WelcomeState::CreateNewUniverse(cnw) = &mut view.state {
                        cnw.picked_path = path;
                        cx.notify();
                    }
                });
            })
            .detach();
        });
        let select_path = div()
            .flex()
            .flex_col()
            .items_end()
            .w_full()
            .child(
                tj_btn(cx, "world_save_location")
                    .label("選択")
                    .w(Self::BUTTON_WIDTH)
                    .on_click(select_fn),
            )
            .child(subtext(
                cnw.picked_path
                    .as_ref()
                    .and_then(|path| path.to_str())
                    .unwrap_or("パスが選択されていません")
                    .to_string(),
            ));
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
                        concatcp!(),
                        concatcp!("作成する", GREATEST_CATHEGORY_TERM, "の名称を設定します"),
                    ))
                    .child(Input::new(&cnw.name_input)),
            )
            .child(tj_border())
            .child(
                description_and_content()
                    .child(title_and_detail(
                        Self::TEXT_WIDTH,
                        "保存場所を選択",
                        concatcp!(
                            "選択したフォルダの下に",
                            GREATEST_CATHEGORY_TERM,
                            "のデータを保存するフォルダが作成されます"
                        ),
                    ))
                    .child(select_path),
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
                        // 戻るボタン
                        tj_scnd_btn(cx, "back-to-main-menu")
                            .label("戻る")
                            .w(Self::BUTTON_WIDTH)
                            .on_click(cx.listener(|view, _, _, _| {
                                view.state = WelcomeState::MainMenu;
                            })),
                    )
                    .child({
                        // 作成ボタン
                        let read = cnw.name_input.read(cx).value();
                        let packed_info: Option<(PathBuf, SharedString)> =
                            (&cnw.picked_path.as_ref())
                                .and_then(|path| (!read.is_empty()).then(|| path))
                                .map(|path| (path.join(read.as_str()), read));
                        tj_btn(cx, "create-universe")
                            .label("作成")
                            .w(Self::BUTTON_WIDTH)
                            .disabled(packed_info.is_none())
                            .on_click(cx.listener(move |view, _, _, cx| {
                                if let Some((path, name)) = &packed_info {
                                    view.worlds.push(WorldRef {
                                        name: name.clone(),
                                        path: path.clone(),
                                    });
                                    emit_global(
                                        GlobalEvent::CreateWorkspace((
                                            path.clone(),
                                            name.to_string(),
                                        )),
                                        cx,
                                    );
                                } else {
                                    emit_global(
                                        GlobalEvent::PopupError("パスは無効です".into()),
                                        cx,
                                    );
                                }
                            }))
                    }),
            )
    }
}
