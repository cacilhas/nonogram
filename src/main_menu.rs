use std::{cell::RefCell, rc::Rc};

use crate::stage::Stage;
use raylib::prelude::*;
use raylib::text::measure_text_ex;

#[derive(Debug)]
pub struct MainMenuStage {
    rect: Rectangle,
    font: Rc<Font>,
}

impl MainMenuStage {
    pub fn new(rect: Rectangle, font: Rc<Font>) -> Self {
        Self { rect, font }
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
        draw.draw_text_ex(
            self.font.as_ref(),
            "Nonogram",
            position,
            84.0,
            2.0,
            Color::DARKCYAN,
        );

        None
    }
}
