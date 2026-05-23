use std::path::PathBuf;

use gpui::{Render, div};

pub struct Workspace {
    episodes: Vec<EpisodeRef>,
}

struct EpisodeRef {
    name: String,
    path: PathBuf,
}

impl Render for Workspace {
    fn render(
        &mut self,
        window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        div()
    }
}
