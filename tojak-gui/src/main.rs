mod components;
mod welcome;
mod workspace;

use gpui::{
    App, Application, Bounds, Entity, WindowBounds, WindowOptions, div, prelude::*, px, size,
};
use gpui_component::Root;

use crate::{
    welcome::{WelcomeEvent, WelcomeScreen},
    workspace::Universe,
};

#[derive(Debug, Clone)]
enum KtStatus {
    Welcome(Entity<WelcomeScreen>),
    Workspace(Entity<Universe>),
}

struct Kakitubata {
    status: KtStatus,
}

impl Kakitubata {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let welcome_entity = cx.new(|_| WelcomeScreen::default());
        cx.subscribe(&welcome_entity, |this, _, event, cx| match event {
            WelcomeEvent::GoToWorkspace(path) => match Universe::from_path(path.clone()) {
                Ok(universe) => this.status = KtStatus::Workspace(cx.new(|_| universe)),
                Err(err) => {
                    eprintln!("[Tojak] ユニバースは破損しています：{}", err);
                }
            },
        })
        .detach();

        Kakitubata {
            status: KtStatus::Welcome(welcome_entity),
        }
    }
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
                let app_view = cx.new(|cx| Kakitubata::new(cx));
                cx.new(|cx| Root::new(app_view, window, cx))
            },
        )
        .unwrap();
    });
}
