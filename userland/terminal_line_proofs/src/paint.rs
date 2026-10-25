// NONOS Operating System (AGPL-3.0-or-later)
//! What the screen is painted with: colours, box strokes and the splash's arch row.

#[path = "../../capsule_terminal/src/term/theme/ansi.rs"]
pub mod ansi;
#[path = "../../capsule_terminal/src/paint/vt/box_arms.rs"]
pub mod box_arms;
#[path = "../../capsule_terminal/src/paint/fetch_arch.rs"]
pub mod fetch_arch;

#[cfg(test)]
mod box_tests;
#[cfg(test)]
mod palette_tests;
