#![feature(let_chains)]
#[macro_use]
extern crate static_init;

pub mod error;
pub mod game;
pub mod ui;

pub mod prelude {
    pub use crate::error;
    pub use crate::game::Board;
    pub use crate::game::Cell;
    pub use crate::ui::fonts;
    pub use crate::ui::stage::main_menu::MainMenuStage;
    pub use crate::ui::stage::Stage;
}
