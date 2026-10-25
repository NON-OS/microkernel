// NONOS Operating System (AGPL-3.0-or-later)
//! The history search: its moves over the real history and line, and what the
//! prompt says while it runs. Mounted side by side, as in the capsule's `term`.

#[path = "../../capsule_terminal/src/term/history/mod.rs"]
pub mod history;
#[path = "../../capsule_terminal/src/term/line/mod.rs"]
pub mod line;
#[path = "../../capsule_terminal/src/term/search.rs"]
pub mod search;
#[path = "../../capsule_terminal/src/paint/search_head.rs"]
pub mod search_head;
#[path = "../../capsule_terminal/src/term/search_step.rs"]
pub mod search_step;

#[cfg(test)]
mod head_tests;
#[cfg(test)]
mod step_tests;
