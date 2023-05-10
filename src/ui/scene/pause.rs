use raylib::prelude::*;
use std::rc::Rc;

use crate::ui::scene::State;

use super::Scene;

pub struct Pause {
    font: Rc<Font>,
    window: Rectangle,
    time_lapse: chrono::Duration,
    threshold: chrono::Duration,
}

impl Default for Pause {
    fn default() -> Self {
        let font = unsafe { ffi::GetFontDefault() };
        let font = unsafe { Font::from_raw(font) };
        Self {
            font: font.into(),
            window: Rectangle::default(),
            time_lapse: chrono::Duration::zero(),
            threshold: chrono::Duration::milliseconds(250),
        }
    }
}

impl Scene for Pause {
    fn init(
        &mut self,
        handle: &mut raylib::RaylibHandle,
        _: &raylib::RaylibThread,
        rect: raylib::prelude::Rectangle,
        font: std::rc::Rc<raylib::text::Font>,
        _: std::rc::Rc<std::cell::RefCell<raylib::prelude::RaylibAudio>>,
    ) {
        handle.set_exit_key(None);
        self.font = font;
        self.window = rect;
    }

    fn update(
        &mut self,
        dt: chrono::Duration,
        handle: &mut RaylibHandle,
        thr: &RaylibThread,
    ) -> super::State {
        if handle.is_key_released(KeyboardKey::KEY_ESCAPE) {
            return State::Previous(2);
        }

        let camera = Camera2D {
            zoom: 1.0,
            ..Default::default()
        };
        let mut base_draw = handle.begin_drawing(thr);
        let mut draw = base_draw.begin_mode2D(camera);

        if self.time_lapse > self.threshold && draw.is_key_released(KeyboardKey::KEY_F3) {
            return State::Previous(1);
        }

        let background_color = Color::WHEAT;
        draw.clear_background(background_color);

        let size = measure_text_ex(self.font.as_ref(), "Nonogram", 84.0, 2.0);
        let position = Vector2::new((self.window.width - size.x) / 2.0, 0.0);
        let bottom = size.y + 64.0;
        draw.draw_text_ex(
            self.font.as_ref(),
            "Nonogram",
            position,
            84.0,
            2.0,
            Color::DARKCYAN,
        );

        let size = measure_text_ex(self.font.as_ref(), "PAUSED", 84.0, 2.0);
        let position = Vector2::new((self.window.width - size.x) / 2.0, bottom);
        let bottom = bottom + size.y + 64.0;
        draw.draw_text_ex(
            self.font.as_ref(),
            "PAUSED",
            position,
            84.0,
            2.0,
            Color::BROWN,
        );

        let size = measure_text_ex(self.font.as_ref(), "F3 get back to game", 32.0, 2.0);
        let position = Vector2::new((self.window.width - size.x) / 2.0, bottom);
        let bottom = bottom + size.y + 32.0;
        draw.draw_text_ex(
            self.font.as_ref(),
            "F3 get back to game",
            position,
            32.0,
            2.0,
            Color::BLACK,
        );

        let size = measure_text_ex(self.font.as_ref(), "ESC abort back to menu", 32.0, 2.0);
        let position = Vector2::new((self.window.width - size.x) / 2.0, bottom);
        draw.draw_text_ex(
            self.font.as_ref(),
            "ESC abort back to menu",
            position,
            32.0,
            2.0,
            Color::BLACK,
        );

        self.time_lapse = self.time_lapse.checked_add(&dt).unwrap();
        State::Keep
    }
}
