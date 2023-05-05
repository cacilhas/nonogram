use std::{cell::RefCell, rc::Rc};

use chrono::Duration;

use raylib::prelude::*;

pub trait Stage {
    #[allow(unused_variables)]
    fn init(&mut self, handle: &mut RaylibHandle, thr: &RaylibThread, rect: Rectangle) {}

    fn update(
        &mut self,
        dt: Duration,
        handle: &mut RaylibHandle,
        thr: &RaylibThread,
    ) -> Option<Rc<RefCell<dyn Stage>>>;
}
