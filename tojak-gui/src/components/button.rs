use gpui::{App, ElementId, prelude::*, px, rgb, rgba};
use gpui_component::{
    Sizable,
    button::{Button, ButtonCustomVariant, ButtonVariants},
};

pub fn tj_btn(cx: &mut App, id: impl Into<ElementId>) -> Button {
    let variant = ButtonCustomVariant::new(cx)
        .color(rgb(0xffffff).into())
        .hover(rgb(0xc0c0c0).into());
    Button::new(id)
        .custom(variant)
        .rounded(px(f32::MAX))
        .small()
        .p_4()
        .text_color(rgb(0x000000))
}

pub fn tj_scnd_btn(cx: &mut App, id: impl Into<ElementId>) -> Button {
    let variant = ButtonCustomVariant::new(cx).border(rgba(0xffffff80).into());
    Button::new(id)
        .custom(variant)
        .rounded(px(f32::MAX))
        .border_1()
        .small()
        .p_4()
        .text_color(rgb(0xffffff))
}
