#[macro_use]
extern crate static_init;

mod audio;
pub mod error;
mod game;
mod ui;

pub mod prelude {
    pub use crate::error;
    pub use crate::game::{Board, Cell};
    pub use crate::ui::{fonts, main_menu::MainMenuScene, resources::Resources};
}
