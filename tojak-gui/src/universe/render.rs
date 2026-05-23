use std::path::Path;

use gpui::{
    AppContext, Context, Div, InteractiveElement, ParentElement, Render, Stateful,
    StatefulInteractiveElement, Styled, div, linear_color_stop, linear_gradient, rgba,
};
use gpui_component::{Icon, scroll::ScrollableElement};

use crate::{
    components::{Horizontal2PanelLayout, SearchBar, subtext, title_and_detail},
    universe::{EventRef, Universe, UniverseRightPanel},
};

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
            .on_click(cx.listener(move |this, _, _, cx| {
                this.change_event_now_on(event_ref.title.clone(), cx);
            }))
        }
    }
}

impl Horizontal2PanelLayout for Universe {
    fn left_panel() -> crate::components::ModDiv<Self>
    where
        Self: Sized,
    {
        |me, original, window, cx| {
            if me.search_bar.is_none() {
                me.search_bar = Some(cx.new(|cx| SearchBar::new(window, cx)));
            }
            let search_bar = me.search_bar.clone().unwrap();
            let create_new = div().flex().flex_col().items_center().gap_2().child(
                div()
                    .child(Icon::empty().path("icons/flag.svg"))
                    .p_2()
                    .id("new-event")
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.create_new_event("無題のイベント".into(), cx);
                    })),
            );

            original
                .px_4()
                .py_2()
                .gap_2()
                .child(search_bar)
                .child(create_new)
                .children(
                    me.get_events(cx)
                        .unwrap_or(vec![])
                        .into_iter()
                        .enumerate()
                        .map(|(i, event_ref)| (me.path.as_path(), i, event_ref))
                        .map(Self::render_event_list(cx)),
                )
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
                        .child(subtext("第一話から始めましょう。")),
                    UniverseRightPanel::EventEdit(event) => {
                        let contents = div()
                            .id("edit-main-scroll")
                            .size_full()
                            .overflow_y_scrollbar()
                            .pt_16()
                            .p_8()
                            .child(div().flex().flex_col().gap_4().children((0..20).map(|i| {
                                div()
                                    .w_full()
                                    .h_32()
                                    .bg(rgba(0xffffff20))
                                    .border_1()
                                    .border_color(rgba(0xffffff60))
                                    .rounded_lg()
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .child(format!("コンテンツブロック {}", i + 1))
                            })));
                        let header = div()
                            .absolute()
                            .top_0()
                            .left_0()
                            .w_full()
                            .h_16()
                            .flex()
                            .items_center()
                            .px_4()
                            .bg(linear_gradient(
                                180.0,
                                linear_color_stop(rgba(0x000000ff), 0.),
                                linear_color_stop(rgba(0x00000000), 1.),
                            ))
                            .child(event.name.clone());
                        div().size_full().relative().child(contents).child(header)
                    }
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
