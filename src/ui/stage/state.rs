use std::{cell::RefCell, rc::Rc};

use super::stage::Stage;

pub enum State {
    Keep,
    New(Rc<RefCell<dyn Stage>>),
    Previous,
}
