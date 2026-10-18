// NONOS Operating System (AGPL-3.0-or-later)
//! What `help` prints and how its rows are laid out, and who signed this terminal.

#[path = "../../capsule_terminal/src/command/builtin/help_layout.rs"]
pub mod help_layout;
#[path = "../../capsule_terminal/src/command/builtin/help_pages.rs"]
pub mod help_pages;
#[path = "../../capsule_terminal/src/command/builtin/tool_list.rs"]
pub mod tool_list;

/// This terminal's entry in the attestation registry, as `whoami`, `version`
/// and the splash say who signed it.
#[path = "../../capsule_terminal/src/command/builtin/receipt/own.rs"]
pub mod receipt_own;

#[cfg(test)]
pub(super) mod help_rows;

#[cfg(test)]
mod help_tests;
#[cfg(test)]
mod help_truth_tests;
#[cfg(test)]
mod tool_tests;
