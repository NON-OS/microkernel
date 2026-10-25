// NONOS Operating System (AGPL-3.0-or-later)
//! A command the desktop shell hands the Terminal from a Launchpad tool tile.

#[path = "../../capsule_terminal/src/term/handed/cadence.rs"]
pub mod handed_cadence;
#[path = "../../capsule_terminal/src/term/handed/parse.rs"]
pub mod handed_parse;
#[path = "../../capsule_terminal/src/term/handed/pick.rs"]
pub mod handed_pick;

#[cfg(test)]
mod handed_tests;
