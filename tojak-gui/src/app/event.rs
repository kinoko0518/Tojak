use gpui::{App, Entity, EventEmitter, Global};
use std::path::PathBuf;

pub enum GlobalEvent {
    PopupError(String),
    CreateWorkspace((PathBuf, String)),
    OpenUniverse(PathBuf),
    BackToWelcome,
}

pub struct GlobalEventBus;

impl EventEmitter<GlobalEvent> for GlobalEventBus {}

pub struct EventBus {
    pub global: Entity<GlobalEventBus>,
    pub universe: Entity<UniverseEventBus>,
}

impl Global for EventBus {}

pub enum UniverseEvent {
    SaveEvent,
    OpenEvent(String),
    CreateEvent(String),
    Rename((String, String)),
}

pub struct UniverseEventBus;

impl EventEmitter<UniverseEvent> for UniverseEventBus {}

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

pub(crate) fn acc_err(error: impl Into<String>, cx: &mut App) {
    emit_global(GlobalEvent::PopupError(error.into()), cx);
}
