use gpui::{App, AppContext, Entity, Render, Window};
use gpui_component::input::{Input, InputEvent, InputState};

pub struct TitleEdit {
    input: Entity<InputState>,
}

impl TitleEdit {
    pub fn new(window: &mut Window, cx: &mut App) -> Self {
        let input = cx.new(|cx| InputState::new(window, cx));
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

        Self { input }
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
        _: &mut gpui::prelude::Context<Self>,
    ) -> impl gpui::prelude::IntoElement {
        Input::new(&self.input).appearance(false)
    }
}
