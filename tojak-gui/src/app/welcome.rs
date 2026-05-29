mod create_new_world;
mod main_menu;
mod render;

use gpui::SharedString;
use std::path::PathBuf;

use create_new_world::CreateNewUniverse;

enum WelcomeState {
    MainMenu,
    CreateNewUniverse(CreateNewUniverse),
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
