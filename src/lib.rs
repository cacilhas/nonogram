//! ![Nonogram](https://github.com/cacilhas/nonogram/raw/master/nonogram.png)
//!
//! A simple random nonogram game.
//!
//! More about it on [Itch.io](https://cacilhas.itch.io/nonogram).
//!
//! # Install
//!
//! ```sh
//! cargo install kodumaro-nonogram
//! ```
//!
//! If you have [UPX](https://upx.github.io/) installed, you maybe want to run:
//!
//! ```sh
//! upx ~/.cargo/bin/nonogram
//! ```
//!
//! # Gameplay
//!
//! Left click (LMB) to set the cell, right click (RMB) or control + LMB to mark a
//! cell as unsettable.
//!
//! You win when you have set all correct cells.
//!
//! # License
//!
//! - [The 3-Clause BSD License](https://opensource.org/licenses/BSD-3-Clause)
//! - [COPYING](https://github.com/cacilhas/nonogram/blob/master/COPYING)

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
