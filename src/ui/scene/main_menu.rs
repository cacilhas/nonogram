use std::cell::RefCell;
use std::rc::Rc;

use crate::game::BoardStruct;

use super::gameplay::GameplayScene;
use super::{Scene, State};
use raylib::prelude::*;

#[derive(Debug)]
pub struct MainMenuScene {
    rect: Rectangle,
    font: Rc<Font>,
    hints: bool,
}

impl Default for MainMenuScene {
    fn default() -> Self {
        let font = unsafe { ffi::GetFontDefault() };
        let font = unsafe { Font::from_raw(font) };
        Self {
            rect: Rectangle::default(),
            font: font.into(),
            hints: false,
        }
    }
}

impl Scene for MainMenuScene {
    fn init(
        &mut self,
        handle: &mut RaylibHandle,
        _: &RaylibThread,
        rect: Rectangle,
        font: Rc<Font>,
        _: Rc<RefCell<RaylibAudio>>,
    ) {
        handle.set_exit_key(Some(KeyboardKey::KEY_ESCAPE));
        self.rect = rect;
        self.font = font;
    }

    fn update(
        &mut self,
        _: chrono::Duration,
        handle: &mut RaylibHandle,
        thr: &RaylibThread,
    ) -> State {
        let clicked = handle.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT);
        let x = handle.get_mouse_x();
        let y = handle.get_mouse_y();
        let mouse = Vector2::new(x as f32, y as f32);

        let camera = Camera2D {
            zoom: 1.0,
            ..Default::default()
        };
        let mut draw = handle.begin_drawing(thr);
        let mut draw = draw.begin_mode2D(camera);

        let background_color = Color::WHEAT;
        draw.clear_background(background_color);

        let size = measure_text_ex(self.font.as_ref(), "Nonogram", 84.0, 2.0);
        let position = Vector2::new((self.rect.width - size.x) / 2.0, 0.0);
        let bottom = size.y + 64.0;
        draw.draw_text_ex(
            self.font.as_ref(),
            "Nonogram",
            position,
            84.0,
            2.0,
            Color::DARKCYAN,
        );

        let size = measure_text_ex(self.font.as_ref(), "5x5", 64.0, 1.0);
        let position = Vector2::new((self.rect.width - size.x) / 2.0, bottom);
        let button_5x5 = Rectangle {
            x: position.x,
            y: position.y,
            width: size.x,
            height: size.y,
        };
        let tint = if button_5x5.check_collision_point_rec(mouse) {
            Color::BLACK
        } else {
            Color::DARKGRAY
        };
        let bottom = bottom + 12.0 + size.y;
        draw.draw_text_ex(self.font.as_ref(), "5x5", position, 64.0, 1.0, tint);

        let size = measure_text_ex(self.font.as_ref(), "10x10", 64.0, 1.0);
        let position = Vector2::new((self.rect.width - size.x) / 2.0, bottom);
        let bottom = bottom + 12.0 + size.y;
        let button_10x10 = Rectangle {
            x: position.x,
            y: position.y,
            width: size.x,
            height: size.y,
        };
        let tint = if button_10x10.check_collision_point_rec(mouse) {
            Color::BLACK
        } else {
            Color::DARKGRAY
        };
        draw.draw_text_ex(self.font.as_ref(), "10x10", position, 64.0, 1.0, tint);

        let size = measure_text_ex(self.font.as_ref(), "15x15", 64.0, 1.0);
        let position = Vector2::new((self.rect.width - size.x) / 2.0, bottom);
        let bottom = bottom + 12.0 + size.y;
        let button_15x15 = Rectangle {
            x: position.x,
            y: position.y,
            width: size.x,
            height: size.y,
        };
        let tint = if button_15x15.check_collision_point_rec(mouse) {
            Color::BLACK
        } else {
            Color::DARKGRAY
        };
        draw.draw_text_ex(self.font.as_ref(), "15x15", position, 64.0, 1.0, tint);

        let size = measure_text_ex(self.font.as_ref(), "Hints", 64.0, 1.0);
        let position = Vector2::new((self.rect.width - size.x) / 2.0, bottom);
        let button_hints = Rectangle {
            x: position.x,
            y: position.y,
            width: size.x,
            height: size.y,
        };
        if self.hints {
            draw.draw_rectangle(
                button_hints.x as i32 - 2,
                button_hints.y as i32,
                button_hints.width as i32 + 4,
                button_hints.height as i32,
                Color::DARKSLATEBLUE,
            );
        }
        let color = if self.hints {
            background_color
        } else {
            Color::DARKSLATEBLUE
        };
        draw.draw_text_ex(self.font.as_ref(), "Hints", position, 64.0, 1.0, color);

        if clicked {
            if button_hints.check_collision_point_rec(mouse) {
                self.hints = !self.hints;
            }

            if button_5x5.check_collision_point_rec(mouse) {
                let board = Box::new(BoardStruct::<5, 5>::random(self.hints));
                return State::New(Rc::new(RefCell::new(GameplayScene::new(board))));
            }

            if button_10x10.check_collision_point_rec(mouse) {
                let board = Box::new(BoardStruct::<10, 10>::random(self.hints));
                return State::New(Rc::new(RefCell::new(GameplayScene::new(board))));
            }

            if button_15x15.check_collision_point_rec(mouse) {
                let board = Box::new(BoardStruct::<15, 15>::random(self.hints));
                return State::New(Rc::new(RefCell::new(GameplayScene::new(board))));
            }
        }

        State::Keep
    }
}
