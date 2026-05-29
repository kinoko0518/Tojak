mod app;
mod components;

pub use app::*;

pub(crate) const LIGHTER_BG: u32 = 0x202020;
pub(crate) const DARKER_BG: u32 = 0x000000;

pub(crate) const BORDER: u32 = 0xffffff20;
pub(crate) const GHOST_TEXT: u32 = 0xffffff40;

pub(crate) const ACCENT: u32 = 0xff7d7d;

use gpui::{App, Application, Bounds, WindowBounds, WindowOptions, prelude::*, px, size};
use gpui_component::Root;

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
