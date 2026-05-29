mod assets;
mod event;
mod universe;
mod welcome;

use crate::{
    ACCENT,
    components::{subtext, tj_btn},
};

use gpui::{Div, Entity, Window, div, prelude::*, rgb};
use time::OffsetDateTime;
use universe::UniverseMeta;

pub use assets::*;
pub use event::*;

pub use universe::Universe;
pub use welcome::WelcomeScreen;

#[derive(Debug, Clone)]
enum KtStatus {
    Welcome,
    Workspace,
}

pub struct Kakitubata {
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
            status: KtStatus::Welcome,
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
