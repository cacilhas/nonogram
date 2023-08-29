use std::{cell::RefCell, fmt, rc::Rc};

use crate::game::{Board, BoardStruct};

use super::{gameplay::GameplayScene, resources::Resources};
use rscenes::prelude::*;

#[derive(Default)]
pub struct MainMenuScene {
    board: Option<Rc<RefCell<dyn Board>>>,
    hints: bool,
}



impl fmt::Debug for MainMenuScene {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "main menu scene")
    }
}

impl Scene<Resources> for MainMenuScene {
    fn init(&mut self, _: &mut RaylibHandle, _: &RaylibThread) -> eyre::Result<()> {
        self.board = None;
        Ok::<(), eyre::Report>(())
    }

    fn update(
        &mut self,
        _: (&mut RaylibHandle, &RaylibThread),
        _: f32,
        _: &mut Resources,
    ) -> eyre::Result<State<Resources>> {
        if let Some(board) = self.board.clone() {
            return Ok::<State<Resources>, eyre::Report>(State::New(Box::new(GameplayScene::new(
                board,
            ))));
        }

        Ok::<State<Resources>, eyre::Report>(State::Keep)
    }

    fn draw(
        &mut self,
        handle: &mut RaylibDrawHandle,
        screen: Rectangle,
        resources: &Resources,
    ) -> eyre::Result<()> {
        let font = resources.font.clone();
        let clicked = handle.is_mouse_button_released(MouseButton::MOUSE_LEFT_BUTTON);
        let x = handle.get_mouse_x();
        let y = handle.get_mouse_y();
        let mouse = Vector2::new(x as f32, y as f32);
        let camera = Camera2D {
            zoom: 1.0,
            ..Default::default()
        };
        let mut draw = handle.begin_mode2D(camera);

        let background_color = colors::WHEAT;
        draw.clear_background(background_color);

        let size = measure_text_ex(font.as_ref(), "Nonogram", 84.0, 2.0);
        let position = Vector2::new((screen.width - size.x) / 2.0, 0.0);
        let bottom = size.y + 64.0;
        draw.draw_text_ex(
            font.as_ref(),
            "Nonogram",
            position,
            84.0,
            2.0,
            colors::DARKCYAN,
        );

        let size = measure_text_ex(font.as_ref(), "5x5", 64.0, 1.0);
        let position = Vector2::new((screen.width - size.x) / 2.0, bottom);
        let button_5x5 = Rectangle {
            x: position.x,
            y: position.y,
            width: size.x,
            height: size.y,
        };
        let tint = if button_5x5.check_collision_point_rec(mouse) {
            colors::BLACK
        } else {
            colors::DARKGRAY
        };
        let bottom = bottom + 12.0 + size.y;
        draw.draw_text_ex(font.as_ref(), "5x5", position, 64.0, 1.0, tint);

        let size = measure_text_ex(font.as_ref(), "10x10", 64.0, 1.0);
        let position = Vector2::new((screen.width - size.x) / 2.0, bottom);
        let bottom = bottom + 12.0 + size.y;
        let button_10x10 = Rectangle {
            x: position.x,
            y: position.y,
            width: size.x,
            height: size.y,
        };
        let tint = if button_10x10.check_collision_point_rec(mouse) {
            colors::BLACK
        } else {
            colors::DARKGRAY
        };
        draw.draw_text_ex(font.as_ref(), "10x10", position, 64.0, 1.0, tint);

        let size = measure_text_ex(font.as_ref(), "15x15", 64.0, 1.0);
        let position = Vector2::new((screen.width - size.x) / 2.0, bottom);
        let bottom = bottom + 12.0 + size.y;
        let button_15x15 = Rectangle {
            x: position.x,
            y: position.y,
            width: size.x,
            height: size.y,
        };
        let tint = if button_15x15.check_collision_point_rec(mouse) {
            colors::BLACK
        } else {
            colors::DARKGRAY
        };
        draw.draw_text_ex(font.as_ref(), "15x15", position, 64.0, 1.0, tint);

        let size = measure_text_ex(font.as_ref(), "Easy", 64.0, 1.0);
        let position = Vector2::new((screen.width - size.x) / 2.0, bottom);
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
                colors::DARKSLATEBLUE,
            );
        }
        let color = if self.hints {
            background_color
        } else {
            colors::DARKSLATEBLUE
        };
        draw.draw_text_ex(font.as_ref(), "Easy", position, 64.0, 1.0, color);

        if clicked {
            if button_hints.check_collision_point_rec(mouse) {
                self.hints = !self.hints;
            }

            if button_5x5.check_collision_point_rec(mouse) {
                self.board = Some(Rc::new(RefCell::new(BoardStruct::<5, 5>::random(
                    self.hints,
                ))));
            }

            if button_10x10.check_collision_point_rec(mouse) {
                self.board = Some(Rc::new(RefCell::new(BoardStruct::<10, 10>::random(
                    self.hints,
                ))));
            }

            if button_15x15.check_collision_point_rec(mouse) {
                self.board = Some(Rc::new(RefCell::new(BoardStruct::<15, 15>::random(
                    self.hints,
                ))));
            }
        } else {
            if draw.is_key_released(KeyboardKey::KEY_E) {
                self.hints = !self.hints;
            }

            if draw.is_key_released(KeyboardKey::KEY_ONE) {
                self.board = Some(Rc::new(RefCell::new(BoardStruct::<5, 5>::random(
                    self.hints,
                ))));
            }

            if draw.is_key_released(KeyboardKey::KEY_TWO) {
                self.board = Some(Rc::new(RefCell::new(BoardStruct::<10, 10>::random(
                    self.hints,
                ))));
            }

            if draw.is_key_released(KeyboardKey::KEY_THREE) {
                self.board = Some(Rc::new(RefCell::new(BoardStruct::<15, 15>::random(
                    self.hints,
                ))));
            }
        }

        Ok::<(), eyre::Report>(())
    }
}
