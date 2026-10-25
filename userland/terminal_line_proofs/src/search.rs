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

// What the prompt says after the path: the branch and the background jobs.
#[path = "../../capsule_terminal/src/term/prompt_bits.rs"]
pub mod prompt_bits;

// Keystroke to screen, as `latency` reports it.
#[path = "../../capsule_terminal/src/term/latency.rs"]
pub mod latency;
// Two tabs side by side: the panes, their sides, and the split as tabs move.
#[path = "../../capsule_terminal/src/term/split.rs"]
pub mod split;
// Where Tab's candidates are shown, and how they are laid out.
#[path = "../../capsule_terminal/src/term/complete_list.rs"]
pub mod complete_list;

#[cfg(test)]
mod complete_tests;
#[cfg(test)]
mod head_tests;
#[cfg(test)]
mod latency_tests;
#[cfg(test)]
mod prompt_tests;
#[cfg(test)]
mod split_tests;
#[cfg(test)]
mod step_tests;
