use std::rc::Rc;

use raylib::ffi;
use raylib::prelude::*;

use crate::game::Board;

use super::Stage;

pub struct GameplayStage {
    board: Box<dyn Board>,
    hhints: Vec<String>,
    vhints: Vec<String>,
    font: Rc<Font>,
    size: Vector2,
    window: Rectangle,
    board_rect: Rectangle,
    hhints_rect: Rectangle,
    vhints_rect: Rectangle,
    cell_size: Vector2,
}

impl GameplayStage {
    pub fn new(board: Box<dyn Board>) -> Self {
        let (w, h) = board.size();
        let size = Vector2::new(w as f32, h as f32);
        let hhints = (0..w)
            .map(|x| {
                board
                    .get_hhint(x)
                    .unwrap()
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            })
            .collect::<Vec<String>>();
        let vhints = (0..h)
            .map(|y| {
                board
                    .get_vhint(y)
                    .unwrap()
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            })
            .collect::<Vec<String>>();
        let font = unsafe { ffi::GetFontDefault() };
        let font = unsafe { Font::from_raw(font) };
        Self {
            board,
            size,
            hhints,
            vhints,
            font: font.into(),
            window: Rectangle::default(),
            board_rect: Rectangle::default(),
            hhints_rect: Rectangle::default(),
            vhints_rect: Rectangle::default(),
            cell_size: Vector2::default(),
        }
    }
}

impl Stage for GameplayStage {
    fn init(
        &mut self,
        handle: &mut raylib::RaylibHandle,
        _: &raylib::RaylibThread,
        rect: raylib::prelude::Rectangle,
        font: Rc<Font>,
    ) {
        handle.set_exit_key(None);
        self.window = rect;
        self.board_rect = Rectangle {
            x: 0.0,
            y: rect.height / 3.0,
            width: rect.width / 1.5,
            height: rect.height / 1.5,
        };
        self.hhints_rect = Rectangle {
            x: 0.0,
            y: 0.0,
            width: rect.width / 1.5,
            height: rect.height / 3.0,
        };
        self.vhints_rect = Rectangle {
            x: rect.width / 1.5,
            y: rect.height / 1.5,
            width: rect.width / 3.0,
            height: rect.height / 3.0,
        };
        self.cell_size = Vector2::new(
            rect.width / (3.0 * self.size.x),
            rect.height / (3.0 * self.size.y),
        );
    }

    fn update(
        &mut self,
        _: chrono::Duration,
        handle: &mut raylib::RaylibHandle,
        thr: &raylib::RaylibThread,
    ) -> Option<Box<dyn Stage>> {
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

        todo!()
    }
}
