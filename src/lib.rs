#![feature(let_chains)]
#[macro_use]
extern crate static_init;

pub mod error;
pub mod fonts;
pub mod main_menu;
pub mod stage;

pub mod prelude {
    pub use crate::error::Error;
    pub use crate::main_menu::MainMenuStage;
    pub use crate::stage::Stage;
}
