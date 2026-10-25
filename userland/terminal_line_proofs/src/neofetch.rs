// NONOS Operating System (AGPL-3.0-or-later)
//! `neofetch`'s pure parts: the figures, the 3D emblem beside the rows, and the
//! colour strips, each with the plain form a file gets.

#[path = "../../capsule_terminal/src/command/builtin/neofetch/compose.rs"]
pub mod compose;
#[path = "../../capsule_terminal/src/command/builtin/neofetch/facts.rs"]
pub mod facts;
#[path = "../../capsule_terminal/src/command/builtin/neofetch/emblem.rs"]
pub mod emblem;
#[path = "../../capsule_terminal/src/command/builtin/neofetch/emblem_cell.rs"]
pub mod emblem_cell;
#[path = "../../capsule_terminal/src/command/builtin/neofetch/emblem_shape.rs"]
pub mod emblem_shape;
#[path = "../../capsule_terminal/src/command/builtin/neofetch/palette.rs"]
pub mod palette;

#[cfg(test)]
mod emblem_tests;
#[cfg(test)]
mod neofetch_tests;
