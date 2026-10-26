// NONOS Operating System (AGPL-3.0-or-later)
//! What the screen is painted with: colours, box strokes and the splash's arch row.

#[path = "../../capsule_terminal/src/term/theme/ansi.rs"]
pub mod ansi;
#[path = "../../capsule_terminal/src/paint/vt/box_arms.rs"]
pub mod box_arms;
#[path = "../../capsule_terminal/src/paint/fetch_arch.rs"]
pub mod fetch_arch;
// The splash banner, drawn into a real paint buffer.
#[path = "../../capsule_terminal/src/paint/fetch_banner.rs"]
pub mod fetch_banner;
// The lift the bars and blocks are drawn on, for the themes' contrast.
#[path = "../../capsule_terminal/src/paint/shade.rs"]
pub mod shade;

#[cfg(test)]
mod banner_tests;
#[cfg(test)]
mod box_tests;
#[cfg(test)]
mod contrast_tests;
#[cfg(test)]
mod palette_tests;
