mod left;
mod right;

use std::path::Path;

use gpui::{Context, Div, InteractiveElement, Render, Stateful, StatefulInteractiveElement};

use crate::{
    EventBus, UniverseEvent,
    components::{Horizontal2PanelLayout, title_and_detail},
};

use super::{EventRef, Universe};

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
        Self::left
    }

    fn right_panel() -> crate::components::ModDiv<Self>
    where
        Self: Sized,
    {
        Self::right
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
