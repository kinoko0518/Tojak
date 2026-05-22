mod components;
mod universe;
mod welcome;

use std::borrow::Cow;

use gpui::{
    App, Application, AssetSource, Bounds, Entity, WindowBounds, WindowOptions, div, prelude::*,
    px, size,
};
use gpui_component::Root;
use include_dir::{Dir, include_dir};

use crate::{
    universe::Universe,
    welcome::{WelcomeEvent, WelcomeScreen},
};

static ASSETS: Dir = include_dir!("$CARGO_MANIFEST_DIR/assets");

struct Assets;

impl AssetSource for Assets {
    fn load(&self, path: &str) -> gpui::Result<Option<Cow<'static, [u8]>>> {
        if let Some(file) = ASSETS.get_file(path) {
            Ok(Some(Cow::Borrowed(file.contents())))
        } else {
            eprintln!("⚠️ Asset not found: {}", path);
            Ok(None)
        }
    }

    fn list(&self, _: &str) -> gpui::Result<Vec<gpui::SharedString>> {
        Ok(vec![])
    }
}

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
                Ok(universe) => {
                    this.status = KtStatus::Workspace(cx.new(|_| universe));
                    cx.notify();
                }
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
        _: &mut gpui::Window,
        _: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        div().size_full().child(match self.status.clone() {
            KtStatus::Welcome(w) => w.into_any_element(),
            KtStatus::Workspace(w) => w.into_any_element(),
        })
    }
}

fn main() {
    Application::new().with_assets(Assets).run(|cx: &mut App| {
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
