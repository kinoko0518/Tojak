mod components;
mod welcome;
mod workspace;

use gpui::{
    App, Application, Bounds, Entity, WindowBounds, WindowOptions, div, prelude::*, px, size,
};
use gpui_component::Root;

use crate::{welcome::WelcomeScreen, workspace::Workspace};

#[derive(Debug, Clone)]
enum KtStatus {
    Welcome(Entity<WelcomeScreen>),
    Workspace(Entity<Workspace>),
}

struct Kakitubata {
    status: KtStatus,
}

impl Render for Kakitubata {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        div().size_full().child(match self.status.clone() {
            KtStatus::Welcome(w) => w.into_any_element(),
            KtStatus::Workspace(w) => w.into_any_element(),
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
            |window, cx| {
                let app_view = cx.new(|cx| {
                    let welcome_entity = cx.new(|_| WelcomeScreen::default());
                    Kakitubata {
                        status: KtStatus::Welcome(welcome_entity),
                    }
                });
                cx.new(|cx| Root::new(app_view, window, cx))
            },
        )
        .unwrap();
    });
}
