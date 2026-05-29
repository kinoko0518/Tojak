use gpui::AssetSource;
use include_dir::{Dir, include_dir};
use std::borrow::Cow;

static ASSETS: Dir = include_dir!("$CARGO_MANIFEST_DIR/assets");

pub struct Assets;

impl AssetSource for Assets {
    fn load(&self, path: &str) -> gpui::Result<Option<Cow<'static, [u8]>>> {
        if let Some(file) = ASSETS.get_file(path) {
            Ok(Some(Cow::Borrowed(file.contents())))
        } else {
            eprintln!("⚠️ Asset not found: {}", path);
            Ok(None)
        }
    }

    fn list(&self, _: &str) -> gpui::Result<Vec<gpui::SharedString>> {
        Ok(vec![])
    }
}
