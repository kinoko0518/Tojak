use gpui::{
    Context, Div, InteractiveElement, ParentElement, StatefulInteractiveElement, Styled, Window,
    div, px, rgba,
};
use gpui_component::Icon;

use super::super::Universe;
use crate::{GHOST_TEXT, GlobalEvent, UniverseEvent, emit_global, emit_universe};

impl Universe {
    pub fn left(&mut self, original: Div, _: &mut Window, cx: &mut Context<Self>) -> Div {
        let search_bar = self.search_bar.clone();
        let create_new = div().flex().flex_col().items_center().gap_2().child(
            div()
                .child(Icon::empty().path("icons/flag.svg"))
                .p_2()
                .id("new-event")
                .on_click(cx.listener(|_, _, _, cx| {
                    emit_universe(UniverseEvent::CreateEvent("無題のイベント".into()), cx);
                })),
        );
        let events = {
            let text = self.search_bar.read(cx).get_text(cx);
            let query = (!text.is_empty()).then(|| text.as_str());
            self.meta.read_events(query)
        };
        let renaming = self.renaming.clone();
        let rename_input = self.rename_input.clone();

        let event_list = match events {
            Ok(events) => div().children(
                events
                    .into_iter()
                    .enumerate()
                    .map(|(i, path)| (self.meta.path.as_path(), i, path))
                    .map(Self::render_event_list(renaming, rename_input, cx)),
            ),
            Err(error) => {
                emit_global(GlobalEvent::PopupError(error.to_string()), cx);
                div()
                    .flex()
                    .flex_col()
                    .child("イベントリストの読み込みに失敗しました：")
                    .child(error.to_string())
                    .text_size(px(10.))
                    .text_color(rgba(GHOST_TEXT))
            }
        };

        original
            .px_4()
            .py_2()
            .gap_2()
            .child(search_bar)
            .child(create_new)
            .child(event_list)
    }
}
