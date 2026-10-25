// NONOS Operating System (AGPL-3.0-or-later)
//! The four themes, so every colour text is drawn in is held to its contrast,
//! and the picker that tries them on the window.

#[path = "../../capsule_terminal/src/term/theme/profiles.rs"]
pub mod profiles;
#[path = "../../capsule_terminal/src/term/theme/types.rs"]
pub mod types;
#[path = "../../capsule_terminal/src/term/theme_pick.rs"]
pub mod theme_pick;

#[cfg(test)]
#[path = "theme_pick_tests.rs"]
mod theme_pick_tests;
