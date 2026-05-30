use super::Universe;
use crate::{
    UniverseEvent,
    components::{tj_border, tj_island},
    consts::{HIGHLIGHT_BG, MODERATE_TEXT, WARNING_RED},
    emit_universe,
};
use gpui::{
    Context, Div, InteractiveElement, ParentElement, Stateful, StatefulInteractiveElement, Styled,
    div, px, rgba,
};

impl Universe {
    fn menu(menu: &'static str) -> Div {
        div()
            .child(menu)
            .cursor_pointer()
            .p_1()
            .rounded_lg()
            .hover(|s| s.bg(gpui::rgb(HIGHLIGHT_BG)))
            .text_color(rgba(MODERATE_TEXT))
    }

    pub(super) fn render_context_menu(&self, cx: &mut Context<Self>) -> Option<Stateful<Div>> {
        let (i, event_ref, pos) = self.context_menu.clone()?;
        let event_title_menu = event_ref.title.clone();

        let item_open = Self::menu("開く")
            .id(("menu-open", i))
            .on_click(cx.listener({
                let title = event_title_menu.clone();
                move |this, _, _, cx| {
                    this.context_menu = None;
                    emit_universe(UniverseEvent::OpenEvent(title.clone()), cx);
                }
            }));
        let item_rename = Self::menu("名前を変更")
            .id(("menu-rename", i))
            .on_click(cx.listener({
                let title = event_title_menu.clone();
                move |this, _, window, cx| {
                    this.context_menu = None;
                    this.renaming = Some((i, title.clone()));
                    this.rename_input.update(cx, |state, cx| {
                        state.set_value(title.clone(), window, cx);
                        state.focus(window, cx);
                    });
                    cx.notify();
                }
            }));
        let item_ep1 = Self::menu("第一話に設定")
            .id(("menu-ep1", i))
            .on_click(cx.listener({
                let title = event_title_menu.clone();
                move |this, _, _, cx| {
                    this.context_menu = None;
                    emit_universe(UniverseEvent::SetEpisodeOne(title.clone()), cx);
                }
            }));
        let item_delete = Self::menu("削除")
            .id(("menu-delete", i))
            .text_color(gpui::rgb(WARNING_RED))
            .on_click(cx.listener({
                let title = event_title_menu.clone();
                move |this, _, _, cx| {
                    this.context_menu = None;
                    emit_universe(UniverseEvent::DeleteEvent(title.clone()), cx);
                }
            }));

        let menu = tj_island()
            .gap_0()
            .id("context-menu")
            .absolute()
            .top(pos.y)
            .left(pos.x)
            .shadow_lg()
            .p_2()
            .text_size(px(13.5))
            .flex()
            .flex_col()
            .w(gpui::px(150.0))
            .on_mouse_down_out(cx.listener(|this, _, _, cx| {
                this.context_menu = None;
                cx.notify();
            }))
            .child(item_open)
            .child(item_rename)
            .child(item_ep1)
            .child(tj_border())
            .child(item_delete);

        Some(menu)
    }
}
