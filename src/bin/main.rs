extern crate nonogram;
use std::rc::Rc;

use chrono::prelude::*;
use nonogram::prelude::*;
use raylib::prelude::*;
use xrandr::XHandle;

fn main() -> anyhow::Result<()> {
    let monitors = XHandle::open()?.monitors()?;
    let (width, height) = monitors
        .iter()
        .find_map(|monitor| {
            if monitor.is_primary {
                Some((monitor.width_px, monitor.height_px))
            } else {
                None
            }
        })
        .ok_or(error!("could not determinate monitor dimensions"))?;
    let screen_rect = Rectangle {
        width: width as f32,
        height: height as f32,
        ..Default::default()
    };
    let (mut handle, thr) = raylib::init()
        .size(width, height)
        .title("nonogram") // WM_CLASS
        .build();
    handle.set_target_fps(30);
    handle.set_window_title(&thr, "Nonogram");
    handle.get_window_state().set_fullscreen_mode(true);
    handle.set_exit_key(Some(KeyboardKey::KEY_ESCAPE));

    let font: Rc<Font> = fonts::get_font(&mut handle, &thr)?.into();
    let mut main_scene = MainMenuStage::default();
    main_scene.init(&mut handle, &thr, screen_rect, font.clone());
    let mut scene: Box<dyn Stage> = Box::new(main_scene);
    let mut tick = Utc::now();

    while !handle.window_should_close() {
        let new_tick = Utc::now();
        if let Some(new_scene) = scene
            .update(new_tick.signed_duration_since(tick), &mut handle, &thr)
            .and_then(|mut scene| {
                scene.init(&mut handle, &thr, screen_rect, font.clone());
                Some(scene)
            })
        {
            scene = new_scene;
        }
        tick = new_tick;
    }
    Ok(())
}
