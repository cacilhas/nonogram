use std::{cell::RefCell, rc::Rc};

use crate::stage::Stage;
use raylib::prelude::*;
use raylib::text::measure_text_ex;

#[derive(Debug)]
pub struct MainMenuStage {
    rect: Rectangle,
    font: Rc<Font>,
    hints: bool,
}

impl MainMenuStage {
    pub fn new(rect: Rectangle, font: Rc<Font>) -> Self {
        Self {
            rect,
            font,
            hints: false,
        }
    }
}

impl Stage for MainMenuStage {
    fn update(
        &mut self,
        _: chrono::Duration,
        draw: &mut RaylibDrawHandle,
    ) -> Option<Rc<RefCell<dyn Stage>>> {
        let camera = Camera2D {
            zoom: 1.0,
            ..Default::default()
        };
        let mut draw = draw.begin_mode2D(camera);
        draw.clear_background(Color::WHEAT);

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
        let bottom = bottom + 12.0 + size.y;
        draw.draw_text_ex(
            self.font.as_ref(),
            "5x5",
            position,
            64.0,
            1.0,
            Color::DARKGRAY,
        );

        let size = measure_text_ex(self.font.as_ref(), "10x10", 64.0, 1.0);
        let position = Vector2::new((self.rect.width - size.x) / 2.0, bottom);
        let bottom = bottom + 12.0 + size.y;
        let button_10x10 = Rectangle {
            x: position.x,
            y: position.y,
            width: size.x,
            height: size.y,
        };
        draw.draw_text_ex(
            self.font.as_ref(),
            "10x10",
            position,
            64.0,
            1.0,
            Color::DARKGRAY,
        );

        let size = measure_text_ex(self.font.as_ref(), "15x15", 64.0, 1.0);
        let position = Vector2::new((self.rect.width - size.x) / 2.0, bottom);
        let bottom = bottom + 12.0 + size.y;
        let button_15x15 = Rectangle {
            x: position.x,
            y: position.y,
            width: size.x,
            height: size.y,
        };
        draw.draw_text_ex(
            self.font.as_ref(),
            "15x15",
            position,
            64.0,
            1.0,
            Color::DARKGRAY,
        );

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
                Color::GREEN,
            );
        }
        let color = if self.hints {
            Color::SLATEBLUE
        } else {
            Color::DARKSLATEBLUE
        };
        draw.draw_text_ex(self.font.as_ref(), "Hints", position, 64.0, 1.0, color);

        None
    }
}
