extern crate nonogram;
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
        .ok_or(Error("could not determinate monitor dimensions".to_owned()))?;
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
    handle.set_exit_key(Some(KeyboardKey::KEY_ESCAPE));

    while !handle.window_should_close() {
        todo!();
    }
    Ok(())
}
