mod operation;
mod paragraph_disp;
mod render;

use gpui::{App, AppContext, Entity, FocusHandle, Window};
use gpui_component::input::InputState;

pub struct Paragraphs {
    pub inputs: Vec<Entity<InputState>>,
    focus_handle: FocusHandle,
}

impl Paragraphs {
    pub fn new(inputs: Vec<Entity<InputState>>, cx: &mut App) -> Self {
        Paragraphs {
            inputs,
            focus_handle: cx.focus_handle(),
        }
    }

    pub fn empty(window: &mut Window, cx: &mut App) -> Entity<Self> {
        let first = cx.new(|cx| InputState::new(window, cx).auto_grow(1, usize::MAX));
        let inputs = vec![first.clone()];

        let this = cx.new(|cx| {
            let mut paragraph = Paragraphs {
                inputs,
                focus_handle: cx.focus_handle(),
            };
            paragraph.subscribe_to_paragraph(&first, window, cx);
            paragraph
        });

        this
    }
}
