mod setup;
pub mod components;
mod update_direction;
mod move_snake;
mod food;
mod collision;
mod tail;
mod score;
pub use setup::setup;
pub use update_direction::*;
pub use components::*;
pub use move_snake::*;
pub use food::*;
pub use tail::*;
pub use collision::*;
pub use score::*;