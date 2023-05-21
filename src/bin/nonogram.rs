extern crate nonogram;
use std::{cell::RefCell, rc::Rc};

use chrono::prelude::*;
use nonogram::prelude::*;
use raylib::prelude::*;

#[cfg(any(target_os = "linux", target_os = "macos"))]
fn main() -> anyhow::Result<()> {
    let (width, height) = resolution::current_resolution()?;
    let screen_rect = Rectangle {
        width: width as f32,
        height: height as f32,
        ..Default::default()
    };
    let (mut handle, thr) = raylib::init()
        .size(width, height)
        .title("nonogram") // WM_CLASS
        .build();
    let audio = Rc::new(RefCell::new(raylib::audio::RaylibAudio::init_audio_device()));
    handle.set_target_fps(30);
    handle.set_window_title(&thr, "Nonogram");
    handle.get_window_state().set_fullscreen_mode(true);
    handle.set_exit_key(Some(KeyboardKey::KEY_ESCAPE));

    let font: Rc<Font> = fonts::get_font(&mut handle, &thr)?.into();
    let mut main_scene = MainMenuScene::default();
    main_scene.init(&mut handle, &thr, screen_rect, font.clone(), audio.clone());
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
                scene.borrow_mut().init(
                    &mut handle,
                    &thr,
                    screen_rect,
                    font.clone(),
                    audio.clone(),
                );
                scenes.push(scene);
            }
            State::Previous(count) => {
                for _ in 0..count {
                    scenes.pop();
                }
                scenes.last().expect("last scene popped").borrow_mut().init(
                    &mut handle,
                    &thr,
                    screen_rect,
                    font.clone(),
                    audio.clone(),
                );
            }
            State::Keep => (),
        }
        tick = new_tick;
    }
    Ok(())
}
