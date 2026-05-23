use gpui::*;
use gpui_component::{
    IconName, IconNamed,
    input::{Input, InputEvent, InputState},
};

pub struct SearchBar {
    input_state: Entity<InputState>,
    on_change: Option<Box<dyn Fn(&str, &mut Context<Self>) + 'static>>,
    on_enter: Option<Box<dyn Fn(&str, &mut Context<Self>) + 'static>>,
}

impl SearchBar {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let input_state = cx.new(|cx| InputState::new(window, cx));

        cx.subscribe(
            &input_state,
            |this: &mut Self, input, event: &InputEvent, cx| {
                let text = input.read(cx).text().to_string();

                match event {
                    InputEvent::Change => {
                        if let Some(callback) = &this.on_change {
                            callback(&text, cx);
                        }
                    }
                    InputEvent::PressEnter { secondary: _ } => {
                        if let Some(callback) = &this.on_enter {
                            callback(&text, cx);
                        }
                    }
                    _ => {}
                }
            },
        )
        .detach();

        Self {
            input_state,
            on_change: None,
            on_enter: None,
        }
    }

    pub fn get_text(&self, cx: &App) -> String {
        self.input_state.read(cx).text().to_string()
    }
}

impl Render for SearchBar {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        Input::new(&self.input_state)
            .appearance(false)
            .prefix(
                div().p_0p5().w_5().h_5().child(
                    svg()
                        .path(IconName::Search.path())
                        .text_color(rgb(0xffffff))
                        .size_full(),
                ),
            )
            .w_full()
            .bg(rgb(0x303030))
            .rounded_full()
    }
}
