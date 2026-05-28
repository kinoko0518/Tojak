use gpui::{
    App, AppContext, Entity, EventEmitter, FocusHandle, Global, InteractiveElement, ParentElement,
    Render, Styled, Window, div,
};
use gpui_component::input::{Input, InputEvent, InputState};

use crate::{emit_universe, universe::UniverseEvent};

pub struct TitleEditBus;

pub enum TitleEditEvent {
    Verify,
}

impl EventEmitter<TitleEditEvent> for TitleEditBus {}

pub struct GlobalTitleEditBus(pub Entity<TitleEditBus>);

impl Global for GlobalTitleEditBus {}

pub fn emit_titleedit(event: TitleEditEvent, cx: &mut App) {
    if cx.has_global::<GlobalTitleEditBus>() {
        let bus = cx.global::<GlobalTitleEditBus>().0.clone();
        bus.update(cx, |_, cx| {
            cx.emit(event);
        });
    }
}

pub struct TitleEdit {
    pub input: Entity<InputState>,
    focus_handle: FocusHandle,
    verified: Option<String>,
}

impl EventEmitter<TitleEditEvent> for TitleEdit {}

impl TitleEdit {
    pub fn new(window: &mut Window, cx: &mut App) -> Entity<Self> {
        if !cx.has_global::<GlobalTitleEditBus>() {
            let bus = cx.new(|_| TitleEditBus);
            cx.set_global(GlobalTitleEditBus(bus));
        }
        let bus = cx.global::<GlobalTitleEditBus>().0.clone();

        let input = cx.new(|cx| InputState::new(window, cx));
        let focus_handle = cx.focus_handle();

        input
            .update(cx, |_, cx| {
                cx.subscribe_in(&input, window, |_, _, event, window, cx| match event {
                    InputEvent::PressEnter { secondary } => {
                        let _ = secondary;
                        window.blur();
                        emit_titleedit(TitleEditEvent::Verify, cx);
                    }
                    _ => (),
                })
            })
            .detach();

        let this = cx.new(|_| Self {
            input,
            focus_handle,
            verified: None,
        });

        this.update(cx, |_, cx| {
            cx.subscribe(&bus, |this, _, event, cx| match event {
                TitleEditEvent::Verify => {
                    let old = this.verified.clone();
                    if let Some(old) = old {
                        let read = this.input.read(cx).text().to_string();
                        this.verified = Some(read.clone());
                        emit_universe(UniverseEvent::Rename((old, read)), cx);
                    }
                }
            })
            .detach();
        });

        this
    }

    pub fn set_field(&mut self, name: String, window: &mut Window, cx: &mut App) {
        self.input
            .update(cx, |state, cx| state.set_value(name.clone(), window, cx));
        self.verified = Some(name);
    }
}

impl Render for TitleEdit {
    fn render(
        &mut self,
        _: &mut gpui::Window,
        cx: &mut gpui::prelude::Context<Self>,
    ) -> impl gpui::prelude::IntoElement {
        div()
            .flex_1()
            .child(Input::new(&self.input).appearance(false))
            .id("title-edit")
            .track_focus(&self.focus_handle)
            .on_mouse_down_out(cx.listener(|_, _, window, _| {
                window.blur();
            }))
    }
}
