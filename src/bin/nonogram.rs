extern crate nonogram;
use std::{cell::RefCell, rc::Rc};

use chrono::prelude::*;
use nonogram::prelude::*;
use raylib::prelude::*;

#[cfg(target_os = "linux")]
use xrandr::XHandle;

#[cfg(target_os = "macos")]
use core_graphics::display::CGDisplay;

#[cfg(any(target_os = "linux", target_os = "macos"))]
fn main() -> anyhow::Result<()> {
    let (width, height) = get_dimensions()?;
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
    let mut main_scene = MainMenuScene::default();
    main_scene.init(&mut handle, &thr, screen_rect, font.clone());
    let mut scenes: Vec<Rc<RefCell<dyn Scene>>> = vec![Rc::new(RefCell::new(main_scene))];
    let mut tick = Utc::now();

    while !handle.window_should_close() {
        let new_tick = Utc::now();
        let state = {
            let scene = scenes.last().expect("no more scenes");
            scene
                .borrow_mut()
                .update(new_tick.signed_duration_since(tick), &mut handle, &thr)
        };
        match state {
            State::New(scene) => {
                {
                    scene
                        .borrow_mut()
                        .init(&mut handle, &thr, screen_rect, font.clone())
                };
                scenes.push(scene);
            }
            State::Previous => {
                scenes.pop();
                scenes
                    .first()
                    .expect("last scene popped")
                    .borrow_mut()
                    .init(&mut handle, &thr, screen_rect, font.clone());
            }
            State::Keep => (),
        }
        tick = new_tick;
    }
    Ok(())
}

#[cfg(target_os = "linux")]
fn get_dimensions() -> anyhow::Result<(i32, i32)> {
    let monitors = XHandle::open()?.monitors()?;
    monitors
        .iter()
        .find_map(|monitor| {
            if monitor.is_primary {
                Some((monitor.width_px, monitor.height_px))
            } else {
                None
            }
        })
        .ok_or_else(|| error!("failed to find monitor resolution").into())
}

#[cfg(target_os = "macos")]
fn get_dimensions() -> anyhow::Result<(i32, i32)> {
    let display = CGDisplay::main();
    let width = display.pixels_wide();
    let height = display.pixels_high();
    Ok((width as i32, height as i32))
}
