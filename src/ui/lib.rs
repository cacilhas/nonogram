#![feature(let_chains)]
#[macro_use]
extern crate static_init;

pub mod error;
pub mod fonts;
pub mod main_menu;
pub mod scene;
pub mod ui;

pub mod prelude {
    pub use crate::error::Error;
    pub use crate::main_menu::MainMenuScene;
    pub use crate::scene::Scene;
}
