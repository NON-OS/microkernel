// NONOS Operating System (AGPL-3.0-or-later)
//! The tables the terminal routes, documents and completes by, mounted as the
//! values the capsule holds, and the tests that read them. Test-only.

#[path = "../../capsule_terminal/src/event/complete_words.rs"]
mod complete_words;
#[path = "../../capsule_terminal/src/event/complete_words_more.rs"]
mod complete_words_more;
#[path = "../../capsule_terminal/src/command/builtin/exit_check.rs"]
mod exit_check;
#[path = "../../capsule_terminal/src/command/builtin/help_usage/mod.rs"]
mod help_usage;
#[path = "../../capsule_terminal/src/command/builtin/nox/route/mod.rs"]
mod nox_route;
#[path = "../../capsule_terminal/src/command/dispatch/route/mod.rs"]
mod shell_route;

/// The one place text is still read: the completion groups and how complete.rs
/// joins them. These are its parts, and parts_tests holds the directory to them.
const WORD_PARTS: &[&str] = &["complete_words.rs", "complete_words_more.rs"];
const COMMANDS_LINE: &str = "const COMMANDS: &[&[&[u8]]] = &[SHELL, FILES, SYSTEM, TEXT, LATE];";

mod completion_tests;
mod help_pages_tests;
mod parts_tests;
mod route_tests;
mod usage_tests;
