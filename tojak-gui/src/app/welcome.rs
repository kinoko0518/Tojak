mod create_new_world;
mod main_menu;
mod render;

use gpui::SharedString;
use std::path::PathBuf;

use create_new_world::CreateNewWorld;

const GREATEST_CATHEGORY_TERM: &str = "ユニバース";

enum WelcomeState {
    MainMenu,
    CreateNewWorld(CreateNewWorld),
}

pub struct WelcomeScreen {
    state: WelcomeState,
    worlds: Vec<WorldRef>,
}

impl Default for WelcomeScreen {
    fn default() -> Self {
        Self {
            state: WelcomeState::MainMenu,
            worlds: Vec::new(),
        }
    }
}

struct WorldRef {
    name: SharedString,
    path: PathBuf,
}
