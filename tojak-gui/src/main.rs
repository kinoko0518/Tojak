mod components;
mod universe;
mod welcome;

use std::{borrow::Cow, path::PathBuf};

use crate::{components::ACCENT, universe::UniverseEvent};
use gpui::{
    App, Application, AssetSource, Bounds, Div, Entity, EventEmitter, Global, Window, WindowBounds,
    WindowOptions, div, prelude::*, px, rgb, size,
};
use gpui_component::Root;
use include_dir::{Dir, include_dir};
use time::OffsetDateTime;

use crate::{
    components::{subtext, tj_btn},
    universe::{Universe, UniverseEventBus, UniverseMeta},
    welcome::WelcomeScreen,
};

static ASSETS: Dir = include_dir!("$CARGO_MANIFEST_DIR/assets");

const EVENT_SUBDIR: &str = "events";

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
    BackToWelcome,
}

struct GlobalEventBus;

impl EventEmitter<GlobalEvent> for GlobalEventBus {}

struct EventBus {
    global: Entity<GlobalEventBus>,
    universe: Entity<UniverseEventBus>,
}

impl Global for EventBus {}

pub(crate) fn emit_global(event: GlobalEvent, cx: &mut App) {
    let bus = cx.global::<EventBus>().global.clone();
    bus.update(cx, |_, cx| {
        cx.emit(event);
    });
}

pub(crate) fn emit_universe(event: UniverseEvent, cx: &mut App) {
    let bus = cx.global::<EventBus>().universe.clone();
    bus.update(cx, |_, cx| {
        cx.emit(event);
    });
}

pub fn acc_err(error: impl Into<String>, cx: &mut App) {
    emit_global(GlobalEvent::PopupError(error.into()), cx);
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
    universe: Option<Entity<Universe>>,
}

impl Kakitubata {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let welcome = cx.new(|_| WelcomeScreen::default());

        let global_bus = cx.global::<EventBus>().global.clone();
        cx.subscribe_in(
            &global_bus,
            window,
            |this, _, event, window, cx| match event {
                GlobalEvent::PopupError(error) => {
                    this.errors.push((OffsetDateTime::now_utc(), error.clone()));
                    cx.notify();
                }
                GlobalEvent::OpenWorkspace(path) => {
                    let meta = UniverseMeta::read_by_path(&path);
                    match meta {
                        Ok(meta) => {
                            let universe = Universe::new(meta, window, cx);
                            this.universe = Some(universe);
                            this.status = KtStatus::Workspace;
                            cx.notify();
                        }
                        Err(error) => {
                            emit_global(GlobalEvent::PopupError(error.to_string()), cx);
                        }
                    }
                }
                GlobalEvent::CreateWorkspace((path, name)) => {
                    let meta = UniverseMeta {
                        path: path.into(),
                        name: name.into(),
                    };
                    let universe = Universe::new(meta, window, cx);
                    universe.update(cx, |this, cx| {
                        this.create_universe(path.clone(), name.clone(), cx)
                    });
                    this.universe = Some(universe);
                    this.status = KtStatus::Workspace;
                    cx.notify();
                }
                GlobalEvent::BackToWelcome => {
                    this.status = KtStatus::Welcome;
                }
            },
        )
        .detach();

        Kakitubata {
            welcome,
            universe: None,
            errors: vec![],
            status: KtStatus::Workspace,
        }
    }

    fn error_screen(cx: &mut Context<Self>) -> Div {
        let back_button = tj_btn(cx, "back-to-welcome")
            .child("ようこそ画面へ戻る")
            .on_click(cx.listener(|_, _, _, cx| {
                emit_global(GlobalEvent::BackToWelcome, cx);
            }))
            .w_64();
        let guidanse = div()
            .size_full()
            .flex()
            .flex_col()
            .content_center()
            .items_center()
            .justify_center()
            .child("おっと、何やらおかしな方法でここに来たようです……")
            .child(subtext("下のボタンから戻りましょう"))
            .child(back_button);
        guidanse
    }
}

impl Render for Kakitubata {
    fn render(
        &mut self,
        _: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        let error_disp =
            div()
                .absolute()
                .bottom_3()
                .right_3()
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .children(self.errors.iter().map(|(_, error)| {
                            div()
                                .p_2()
                                .w_80()
                                .bg(rgb(ACCENT))
                                .rounded_lg()
                                .child(error.clone())
                        })),
                );
        let main = match &self.status {
            KtStatus::Welcome => self.welcome.clone().into_any_element(),
            KtStatus::Workspace => match &self.universe {
                Some(universe) => universe.clone().into_any_element(),
                None => Self::error_screen(cx).into_any_element(),
            },
        };
        div().size_full().child(main).relative().child(error_disp)
    }
}

fn main() {
    Application::new().with_assets(Assets).run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(800.), px(500.0)), cx);
        gpui_component::init(cx);

        let global_bus = cx.new(|_| GlobalEventBus);
        let universe_bus = cx.new(|_| UniverseEventBus);

        cx.set_global(EventBus {
            global: global_bus,
            universe: universe_bus,
        });

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
