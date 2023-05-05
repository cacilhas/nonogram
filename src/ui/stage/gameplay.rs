use crate::game::Board;

use super::Stage;

pub struct GameplayStage {
    board: Box<dyn Board>,
}

impl GameplayStage {
    pub fn new(board: Box<dyn Board>) -> Self {
        Self { board }
    }
}

impl Stage for GameplayStage {
    fn init(
        &mut self,
        handle: &mut raylib::RaylibHandle,
        thr: &raylib::RaylibThread,
        rect: raylib::prelude::Rectangle,
    ) {
        todo!()
    }

    fn update(
        &mut self,
        dt: chrono::Duration,
        handle: &mut raylib::RaylibHandle,
        thr: &raylib::RaylibThread,
    ) -> Option<Box<dyn Stage>> {
        todo!()
    }
}
