// NONOS Operating System (AGPL-3.0-or-later)
//! `market list`'s rows, as pacman prints a search.

#[path = "../../capsule_terminal/src/command/builtin/market/list_row.rs"]
pub mod list_row;

/// The installs the Terminal follows until it can say how they ended.
#[path = "../../capsule_terminal/src/command/builtin/market/watch.rs"]
pub mod watch;

#[cfg(test)]
mod market_tests;
#[cfg(test)]
mod watch_tests;
