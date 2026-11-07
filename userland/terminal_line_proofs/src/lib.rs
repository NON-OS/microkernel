// NONOS Operating System (AGPL-3.0-or-later)
//! Host proofs for the terminal line editor. Includes the real capsule source
//! via `#[path]` so the code under test is the code that ships.

// The capsule sources are no_std and reach the heap through `alloc`. Declaring
// it here lets them compile unchanged against the host's std.
extern crate alloc;

/// The capsule's own module layout, mirrored so its sources compile unchanged.
/// Everything under here is either the real file or the one constant the
/// capsule reads from its own dimensions.
pub mod term;

// Declared at the crate root: a `#[path]` inside nested inline modules
// resolves against the directory those modules imply, not against this file.
#[path = "../../capsule_terminal/src/term/scrollback/mod.rs"]
pub mod scrollback;
#[cfg(test)]
mod scrollback_fed_tests;

#[path = "../../capsule_terminal/src/term/util/format_u64.rs"]
pub mod fmt_u64;

// The command LANGUAGE core now ships in the nonos_shell library; these mounts
// drive the library's real files, so the proofs still exercise the code that
// ships. The moved files name their helpers by the library's own crate paths
// (`crate::flags`, `crate::util`), so those are provided here at the proof
// crate root for the mounts to resolve against.
#[path = "../../nonos_shell/src/flags/mod.rs"]
pub mod flags_inner;

pub use crate::flags_inner as flags;

#[path = "../../nonos_shell/src/util/is_space.rs"]
mod util_is_space;
#[path = "../../nonos_shell/src/util/format_u64.rs"]
mod util_format_u64;

pub mod util {
    pub use crate::util_format_u64::format_u64;
    pub use crate::util_is_space::is_space;
}

pub mod command {
    pub use crate::flags_inner as flags;
}

/// `qwen`'s pure parts and their proofs.
pub mod qwen;

// The mounts by topic, each group with its tests. The capsule sources name one
// another by `crate::` paths, so every mount is re-exported here under the
// name it had at the root.
mod filters;
mod handed;
mod help;
mod input_line;
mod jobs;
pub mod market;
mod neighbours;
pub mod neofetch;
#[cfg(test)]
mod net_again;
mod network;
mod paint;
pub mod term_search;
mod shell;
pub mod theme;

pub use filters::{input, tree_render};
pub use handed::{handed_cadence, handed_parse, handed_pick};
pub use help::{help_layout, help_pages, hint_sets, receipt_own, tool_list};
pub use input_line::{cooked, cooked_kill, expand, line, line_chars, line_window, select};
pub use jobs::{
    agentproof_job, box_app, box_hold, box_log, box_missing, box_screen, install_follow, kill_target,
};
pub use jobs::{linux_frames, pkg_handoff, tool_argv, tool_busy, tool_service};
pub use neighbours::{cadence, goto_line, quick_open};
pub use network::{direct_gate, direct_only, exchange_wait, git_ask, offline_gate};
pub use paint::{ansi, box_arms, fetch_arch, fetch_banner, shade};
pub use shell::{parse, redirect, statements, suggest, tool_admit};

#[cfg(test)]
pub use filters::{count, grep_paint, text};
#[cfg(test)]
use help::help_rows;

#[cfg(test)]
mod tables;
