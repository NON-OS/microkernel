// NONOS Operating System (AGPL-3.0-or-later)
//! The input line: its editor, scroll window, history expansion and cooked editing.

#[path = "../../capsule_terminal/src/term/line/mod.rs"]
pub mod line;

// The input line's scroll window and how it reads the line's bytes as
// characters, side by side where the window finds `super::line_chars`.
#[path = "../../capsule_terminal/src/paint/line_chars.rs"]
pub mod line_chars;
#[path = "../../capsule_terminal/src/paint/line_window.rs"]
pub mod line_window;

#[path = "../../capsule_terminal/src/term/history/expand.rs"]
pub mod expand;

// The line editing a foreground program gets, and the pointer selection.
#[path = "../../capsule_terminal/src/event/cooked.rs"]
pub mod cooked;
#[path = "../../capsule_terminal/src/event/cooked_kill.rs"]
pub mod cooked_kill;
#[path = "../../capsule_terminal/src/term/select.rs"]
pub mod select;

#[cfg(test)]
mod cooked_tests;
#[cfg(test)]
mod expand_tests;
#[cfg(test)]
mod line_tests;
#[cfg(test)]
mod line_utf8_tests;
#[cfg(test)]
mod long_line_tests;
#[cfg(test)]
mod word_kill_tests;
