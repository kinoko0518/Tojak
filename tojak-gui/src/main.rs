mod components;
mod welcome;

use gpui::{App, Application, Bounds, WindowBounds, WindowOptions, div, prelude::*, px, size};

#[derive(Default)]
enum KtStatus {
    #[default]
    Welcome,
}

struct Kakitubata {
    status: KtStatus,
}

impl Default for Kakitubata {
    fn default() -> Self {
        Self {
            status: KtStatus::default(),
        }
    }
}

impl Render for Kakitubata {
    fn render(
        &mut self,
        _: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        div().size_full().child(match self.status {
            KtStatus::Welcome => self.render_welcome(cx),
        })
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(800.), px(500.0)), cx);
        gpui_component::init(cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_| Kakitubata::default()),
        )
        .unwrap();
    });
}
