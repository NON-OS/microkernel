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
#[path = "../../capsule_terminal/src/term/util/format_u64.rs"]
pub mod fmt_u64;

#[path = "../../capsule_terminal/src/command/flags/mod.rs"]
pub mod flags_inner;

pub mod command {
    pub use crate::flags_inner as flags;
}

// The input line's scroll window and how it reads the line's bytes as
// characters, at the crate root where the window finds `super::line_chars`.
#[path = "../../capsule_terminal/src/paint/fetch_arch.rs"]
pub mod fetch_arch;
#[path = "../../capsule_terminal/src/paint/line_chars.rs"]
pub mod line_chars;
#[path = "../../capsule_terminal/src/paint/line_window.rs"]
pub mod line_window;

/// `qwen`'s pure parts and their proofs.
pub mod qwen;

/// The pipeline text filters. `pub(super)` in the capsule, which resolves to
/// crate-visible here, so the tests can reach them.
///
/// Everything in the module is reached from the tests and from nowhere else,
/// so on a plain library build it reads as dead. It is not: the capsule calls
/// all of it. The allowance covers the shape of this crate, not a real gap.
#[allow(dead_code)]
#[path = "../../capsule_terminal/src/command/dispatch/filter/text.rs"]
pub mod text;

/// Each filter's flag table, the lines a file gives a filter, and what a pipe
/// stage that is not a filter says. `text` reads its flags from here.
#[path = "../../capsule_terminal/src/command/dispatch/filter/input.rs"]
pub mod input;

/// `wc`, `head` and `tail` over piped lines; crate-private in the capsule, so
/// compiled for the tests alone.
#[cfg(test)]
#[path = "../../capsule_terminal/src/command/dispatch/filter/count.rs"]
pub mod count;

/// This terminal's entry in the attestation registry, as `whoami`, `version`
/// and the splash say who signed it.
#[path = "../../capsule_terminal/src/command/builtin/receipt/own.rs"]
pub mod receipt_own;

#[path = "../../capsule_terminal/src/term/line/mod.rs"]
pub mod line;

#[path = "../../capsule_terminal/src/command/builtin/fs/tree_render.rs"]
pub mod tree_render;

#[path = "../../capsule_terminal/src/term/history/expand.rs"]
pub mod expand;

#[path = "../../capsule_terminal/src/command/suggest.rs"]
pub mod suggest;

// How a typed line becomes words and statements, and what its redirects ask
// for: the tokenizer, the `;`/`&&`/`&` split, and the redirect plan the
// dispatch and a tool's job both read.
#[path = "../../capsule_terminal/src/command/parse/mod.rs"]
pub mod parse;
#[path = "../../capsule_terminal/src/command/dispatch/statements.rs"]
pub mod statements;
#[path = "../../capsule_terminal/src/command/dispatch/redirect.rs"]
pub mod redirect;
#[path = "../../capsule_terminal/src/command/dispatch/tool_admit.rs"]
pub mod tool_admit;

// The editor shares this crate rather than growing a second proofs crate for
// two pure functions.
#[path = "../../capsule_text_editor/src/editor/goto_line.rs"]
pub mod goto_line;

#[path = "../../capsule_text_editor/src/editor/quick_open.rs"]
pub mod quick_open;

#[path = "../../capsule_net_core/src/server/runner/cadence.rs"]
pub mod cadence;

// Reached only from the tests on a plain library build; the capsule calls it
// from grep_scan. Same crate-shape allowance as the filters above.
#[allow(dead_code)]
#[path = "../../capsule_terminal/src/command/builtin/fs/grep_paint.rs"]
pub mod grep_paint;

// The line editing a foreground program gets, and the pointer selection.
#[path = "../../capsule_terminal/src/event/cooked.rs"]
pub mod cooked;
#[path = "../../capsule_terminal/src/event/cooked_kill.rs"]
pub mod cooked_kill;
#[path = "../../capsule_terminal/src/command/builtin/tool_busy.rs"]
pub mod tool_busy;
#[path = "../../capsule_terminal/src/command/builtin/nox/kill_target.rs"]
pub mod kill_target;
#[path = "../../capsule_terminal/src/term/select.rs"]
pub mod select;

// What `help` prints and how its rows are laid out.
#[path = "../../capsule_terminal/src/command/builtin/help_layout.rs"]
pub mod help_layout;
#[path = "../../capsule_terminal/src/command/builtin/help_pages.rs"]
pub mod help_pages;
#[path = "../../capsule_terminal/src/command/builtin/tool_list.rs"]
pub mod tool_list;

// The colours programs name, and the box characters stroked per cell.
#[path = "../../capsule_terminal/src/term/theme/ansi.rs"]
pub mod ansi;
#[path = "../../capsule_terminal/src/paint/vt/box_arms.rs"]
pub mod box_arms;

// The commands that can only leave directly, and the rule that lets them:
// nonos_route_link's, the one every capsule holding Network goes by.
#[path = "../../capsule_terminal/src/command/builtin/direct_gate.rs"]
pub mod direct_gate;
#[path = "../../nonos_route_link/src/direct_only.rs"]
pub mod direct_only;
// What those commands say, before anything is sent, when the machine has
// no address: no cable and no Wi-Fi joined.
#[path = "../../capsule_terminal/src/command/builtin/offline_gate.rs"]
pub mod offline_gate;

// What a network job makes of the far end's silence each tick, and the one
// request a stepped `git clone` holds for its job to carry.
#[path = "../../capsule_terminal/src/mixnet/exchange/wait.rs"]
pub mod exchange_wait;
#[path = "../../capsule_terminal/src/git/transport/ask.rs"]
pub mod git_ask;

// A load `install` stopped waiting on, followed from the job's ticks.
#[path = "../../capsule_terminal/src/command/builtin/nox/install/follow.rs"]
pub mod install_follow;

// The answer a `pkg` worker thread hands back to the window thread.
#[path = "../../libc/src/thread/handoff.rs"]
pub mod pkg_handoff;

// A command the desktop shell hands the Terminal from a Launchpad tool tile:
// what the reply holds, which tab takes it, and how often the window asks.
#[path = "../../capsule_terminal/src/term/handed/cadence.rs"]
pub mod handed_cadence;
#[path = "../../capsule_terminal/src/term/handed/parse.rs"]
pub mod handed_parse;
#[path = "../../capsule_terminal/src/term/handed/pick.rs"]
pub mod handed_pick;

#[cfg(test)]
mod box_tests;
#[cfg(test)]
mod cadence_tests;
#[cfg(test)]
mod cooked_tests;
#[cfg(test)]
mod direct_gate_tests;
#[cfg(test)]
mod expand_tests;
#[cfg(test)]
mod filter_tests;
#[cfg(test)]
mod goto_tests;
#[cfg(test)]
mod grep_paint_tests;
#[cfg(test)]
mod handed_tests;
#[cfg(test)]
mod help_rows;
#[cfg(test)]
mod help_tests;
#[cfg(test)]
mod help_truth_tests;
#[cfg(test)]
mod install_follow_tests;
#[cfg(test)]
mod kill_target_tests;
#[cfg(test)]
mod tool_busy_tests;
#[cfg(test)]
mod line_tests;
#[cfg(test)]
mod line_utf8_tests;
#[cfg(test)]
mod long_line_tests;
#[cfg(test)]
mod network_job_tests;
#[cfg(test)]
mod offline_gate_tests;
#[cfg(test)]
mod palette_tests;
#[cfg(test)]
mod pipe_filter_tests;
#[cfg(test)]
mod pkg_handoff_tests;
#[cfg(test)]
mod quick_open_tests;
#[cfg(test)]
mod redirect_tests;
#[cfg(test)]
mod suggest_tests;
#[cfg(test)]
mod tool_tests;
#[cfg(test)]
mod tree_tests;
#[cfg(test)]
mod usage_tests;
#[cfg(test)]
mod word_kill_tests;
