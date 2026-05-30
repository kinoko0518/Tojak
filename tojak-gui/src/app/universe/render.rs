mod context_menu;
mod left;
mod right;

use std::path::Path;

use gpui::{
    Context, Div, InteractiveElement, ParentElement, Render, Stateful, StatefulInteractiveElement,
    Styled, div,
};

use crate::{
    UniverseEvent,
    components::{Horizontal2PanelLayout, title_and_detail},
    emit_universe,
};

use super::{EventRef, Universe};

gpui::actions!(universe, [ParagraphBackspace]);

impl Universe {
    fn render_event_list<'a>(
        renaming: Option<(usize, String)>,
        rename_input: gpui::Entity<gpui_component::input::InputState>,
        cx: &'a mut Context<Self>,
    ) -> impl FnMut((&Path, usize, EventRef)) -> Stateful<Div> + 'a {
        move |(base, i, event_ref)| {
            let is_renaming = renaming.as_ref().map(|(idx, _)| *idx == i).unwrap_or(false);

            let mut el = div().id(("list-of-event", i)).relative();

            if is_renaming {
                el = el.child(
                    div().p_2().child(
                        gpui_component::input::Input::new(&rename_input)
                            .appearance(false)
                            .w_full(),
                    ),
                );
            } else {
                let event_title = event_ref.title.clone();
                let event_title2 = event_title.clone();
                el = el.child(
                    title_and_detail(
                        Self::TEXT_WIDTH,
                        event_title2.clone(),
                        event_ref
                            .path
                            .strip_prefix(base)
                            .ok()
                            .and_then(|path| path.to_str())
                            .unwrap_or("パスの取得に失敗しました")
                            .to_string(),
                    )
                    .id(("event-title", i))
                    .on_click(cx.listener(move |_, _, _, cx| {
                        emit_universe(UniverseEvent::OpenEvent(event_title2.clone()), cx);
                    })),
                );
            }

            let event_ref_menu = event_ref.clone();
            el = el.on_mouse_down(
                gpui::MouseButton::Right,
                cx.listener(move |this, ev: &gpui::MouseDownEvent, _, cx| {
                    this.context_menu = Some((i, event_ref_menu.clone(), ev.position));
                    cx.notify();
                }),
            );

            el
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
        let layout = self.render_layout(window, cx);
        let mut wrapper = div().size_full().child(layout);

        if let Some(menu) = self.render_context_menu(cx) {
            wrapper = wrapper.child(menu);
        }

        wrapper
    }
}
