use gpui::{
    App, AppContext, Entity, FocusHandle, InteractiveElement, ParentElement, Render,
    Styled, Window, div,
};
use gpui_component::input::{Input, InputEvent, InputState};



pub struct TitleEdit {
    pub input: Entity<InputState>,
    focus_handle: FocusHandle,
}



impl TitleEdit {
    pub fn new(window: &mut Window, cx: &mut App) -> Self {
        let input = cx.new(|cx| InputState::new(window, cx));
        let focus_handle = cx.focus_handle();

        input
            .update(cx, |_, cx| {
                cx.subscribe_in(&input, window, |_, _, event, window, _| match event {
                    InputEvent::PressEnter { secondary } => {
                        let _ = secondary;
                        window.blur();
                    }
                    _ => (),
                })
            })
            .detach();

        Self {
            input,
            focus_handle,
        }
    }

    pub fn set_field(&mut self, name: String, window: &mut Window, cx: &mut App) {
        self.input
            .update(cx, |state, cx| state.set_value(name.clone(), window, cx));
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
