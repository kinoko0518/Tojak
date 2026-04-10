use std::path::PathBuf;

use ayame::AbstractAozoraZip;
use chrono::Utc;
use uuid::Uuid;

struct Block {
    inside: String,
    uuid: Uuid,
    hash: u64,
}

impl Block {
    pub fn xhtmls(&mut self) {
        AbstractAozoraZip {
            text: self.inside.as_str(),
            dependencies: Dependencies::default(),
        }
    }
}

pub struct Episode {
    pub title: String,
    pub last_edit: Utc,
}

pub struct World {
    path: PathBuf,
}
