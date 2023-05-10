use std::{cell::RefCell, rc::Rc};

use super::state::State;

use chrono::Duration;

use raylib::prelude::*;

pub trait Scene {
    #[allow(unused_variables)]
    fn init(
        &mut self,
        handle: &mut RaylibHandle,
        thr: &RaylibThread,
        rect: Rectangle,
        font: Rc<Font>,
        audio: Rc<RefCell<RaylibAudio>>,
    ) {
    }

    fn update(&mut self, dt: Duration, handle: &mut RaylibHandle, thr: &RaylibThread) -> State;
}
