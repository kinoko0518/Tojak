mod components;
mod universe;
mod welcome;
mod work;

use std::{borrow::Cow, path::PathBuf};

use gpui::{
    App, Application, AssetSource, Bounds, Entity, Window, WindowBounds, WindowOptions, div,
    prelude::*, px, rgb, size,
};
use gpui_component::Root;
use include_dir::{Dir, include_dir};
use time::OffsetDateTime;

use crate::{universe::Universe, welcome::WelcomeScreen};

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

enum GlobalEvent {
    PopupError(String),
    CreateWorkspace((PathBuf, String)),
    OpenWorkspace(PathBuf),
}

#[derive(Debug, Clone)]
enum KtStatus {
    Welcome,
    Workspace,
}

struct Kakitubata {
    status: KtStatus,

    errors: Vec<(time::OffsetDateTime, String)>,

    welcome: Entity<WelcomeScreen>,
    universe: Entity<Universe>,
}

impl Kakitubata {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let welcome = cx.new(|_| WelcomeScreen::default());
        let universe = Universe::new(window, cx);
        cx.subscribe_in(&welcome, window, |this, _, event, window, cx| match event {
            GlobalEvent::PopupError(error) => {
                this.errors.push((OffsetDateTime::now_utc(), error.clone()));
                cx.notify();
            }
            GlobalEvent::OpenWorkspace(path) => {
                this.universe.update(cx, |this, cx| {
                    if let Err(error) = this.open_universe(path, window, cx) {
                        cx.emit(GlobalEvent::PopupError(error.to_string()));
                    }
                });
                this.status = KtStatus::Workspace;
                cx.notify();
            }
            GlobalEvent::CreateWorkspace((path, name)) => {
                this.universe.update(cx, |this, cx| {
                    this.create_universe(path.clone(), name.clone(), cx);
                });
                this.status = KtStatus::Workspace;
                cx.notify();
            }
        })
        .detach();

        Kakitubata {
            welcome,
            universe,
            errors: vec![],
            status: KtStatus::Welcome,
        }
    }
}

impl Render for Kakitubata {
    fn render(
        &mut self,
        _: &mut gpui::Window,
        _: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        div()
            .size_full()
            .child(match &self.status {
                KtStatus::Welcome => self.welcome.clone().into_any_element(),
                KtStatus::Workspace => self.universe.clone().into_any_element(),
            })
            .relative()
            .child(
                div()
                    .absolute()
                    .bottom_3()
                    .right_3()
                    .flex()
                    .flex_col()
                    .children(self.errors.iter().map(|(_, error)| {
                        div().bg(rgb(0xf24680)).rounded_lg().child(error.clone())
                    })),
            )
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
                let app_view = cx.new(|cx| Kakitubata::new(window, cx));
                cx.new(|cx| Root::new(app_view, window, cx))
            },
        )
        .unwrap();
    });
}
