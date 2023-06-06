use std::{cell::RefCell, rc::Rc};

use rscenes::prelude::*;

pub struct Resources {
    pub audio: Rc<RefCell<RaylibAudio>>,
    pub font: Rc<Font>,
}

impl Default for Resources {
    fn default() -> Self {
        let font = unsafe { ffi::GetFontDefault() };
        let font = unsafe { Font::from_raw(font) };

        Self {
            audio: Rc::new(RefCell::new(RaylibAudio::init_audio_device())),
            font: Rc::new(font),
        }
    }
}
