use gpui::{Context, Div, IntoElement, ParentElement, Styled, Window, div};

use super::super::{Universe, UniverseRightPanel};
use crate::components::subtext;

impl Universe {
    pub fn right(&mut self, original: Div, _: &mut Window, _: &mut Context<Self>) -> Div {
        original.child({
            match &self.right_panel {
                UniverseRightPanel::Empty => div()
                    .flex()
                    .flex_col()
                    .justify_center()
                    .items_center()
                    .child("まだ何も開いていません")
                    .child(subtext("第一話から始めましょう。"))
                    .into_any_element(),
                UniverseRightPanel::EventEdit => self.editor.clone().into_any_element(),
            }
        })
    }
}
