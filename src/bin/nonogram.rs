extern crate kodumaro_nonogram;

use kodumaro_nonogram::prelude::*;
use rscenes::prelude::*;

#[cfg(any(target_os = "linux", target_os = "macos"))]
fn main() -> anyhow::Result<()> {
    let (width, height) = resolution::current_resolution()?;
    let mut builder = raylib::init();
    builder
        .size(width, height)
        .title("nonogram") // WM_CLASS
        .build();
    let mut manager = SceneManager::new(builder, Resources::default());
    manager.config(|handle, thread, resources| {
        handle.set_target_fps(30);
        handle.set_window_title(&thread, "Nonogram");
        handle.get_window_state().set_fullscreen_mode(true);
        resources.font = fonts::get_font(handle, thread)?.into();
        anyhow::Ok(())
    })?;
    manager.add_first_scene(Box::new(MainMenuScene::default()));
    manager.start()
}
