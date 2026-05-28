use std::path::Path;

use gpui::{
    Context, Div, InteractiveElement, IntoElement, ParentElement, Render, Stateful,
    StatefulInteractiveElement, Styled, div, px, rgba,
};
use gpui_component::Icon;

use crate::{
    EventBus, GlobalEvent, emit_global, emit_universe,
    components::{GHOST_TEXT, Horizontal2PanelLayout, subtext, title_and_detail},
    universe::{EventRef, Universe, UniverseEvent, UniverseRightPanel},
};

gpui::actions!(universe, [ParagraphBackspace]);

impl Universe {
    fn render_event_list(
        cx: &mut Context<Self>,
    ) -> impl FnMut((&Path, usize, EventRef)) -> Stateful<Div> {
        |(base, i, event_ref)| {
            title_and_detail(
                Self::TEXT_WIDTH,
                event_ref.title.clone(),
                event_ref
                    .path
                    .strip_prefix(base)
                    .ok()
                    .and_then(|path| path.to_str())
                    .unwrap_or("パスの取得に失敗しました")
                    .to_string(),
            )
            .id(("list-of-event", i))
            .on_click(cx.listener(move |_, _, _, cx| {
                let bus = cx.global::<EventBus>().universe.clone();
                bus.update(cx, |_, cx| {
                    cx.emit(UniverseEvent::OpenEvent(event_ref.title.clone()));
                })
            }))
        }
    }
}

impl Horizontal2PanelLayout for Universe {
    fn left_panel() -> crate::components::ModDiv<Self>
    where
        Self: Sized,
    {
        |me, original, _, cx| {
            let search_bar = me.search_bar.clone();
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
                let text = me.search_bar.read(cx).get_text(cx);
                let query = (!text.is_empty()).then(|| text.as_str());
                me.meta.read_events(query)
            };
            let event_list = match events {
                Ok(events) => div().children(
                    events
                        .into_iter()
                        .enumerate()
                        .map(|(i, path)| (me.meta.path.as_path(), i, path))
                        .map(Self::render_event_list(cx)),
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

    fn right_panel() -> crate::components::ModDiv<Self>
    where
        Self: Sized,
    {
        |me, original, _, _| {
            original.child({
                match &me.right_panel {
                    UniverseRightPanel::Empty => div()
                        .flex()
                        .flex_col()
                        .justify_center()
                        .items_center()
                        .child("まだ何も開いていません")
                        .child(subtext("第一話から始めましょう。"))
                        .into_any_element(),
                    UniverseRightPanel::EventEdit => me.editor.clone().into_any_element(),
                }
            })
        }
    }
}

impl Render for Universe {
    fn render(
        &mut self,
        window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        self.render_layout(window, cx)
    }
}
