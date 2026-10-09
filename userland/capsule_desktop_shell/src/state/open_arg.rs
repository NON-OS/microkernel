// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! What an app's OP_TAKE_OPEN_ARG answer can hold, kept apart from the IPC so
//! it is proven on the host (desktop_proofs open_arg_tests).
//!
//! Two things travel there, and they cannot be mistaken for each other:
//!
//! - a path, for the editor, Files, the viewer and the players. It always
//!   starts with '/': OP_OPEN_WITH refuses anything else, and the desktop's
//!   own icons hand full paths.
//! - a command line, for the Terminal only, from a Launchpad tool tile:
//!   `run:<line>` to run it as typed, or `type:<line>` to leave it on the
//!   prompt for the person to finish. No path starts with either word, and
//!   the Terminal (term/handed/parse.rs) runs nothing that lacks one.
//!
//! The shell keeps the command apart from the paths (`Context::pending_command`)
//! and answers it only to a window of the Terminal, so no other process can
//! put a command there: OP_OPEN_WITH, the one way another process leaves an
//! argument, can only leave a path.

use alloc::string::String;

/// Hand-synced with the Terminal's term/handed/parse.rs.
pub const RUN: &str = "run:";
pub const TYPE: &str = "type:";

/// The only app a command is handed to.
pub const TERMINAL: &[u8] = b"app.terminal";

/// How long a command waits for the Terminal. A tile launches the Terminal
/// and its window asks within a few ticks of starting; a command still there
/// long after was for a window that never came up, and must not run in the
/// next Terminal someone opens for something else.
pub const COMMAND_TTL_MS: i64 = 10_000;

/// Tools whose run with no arguments prints their usage: pastel shows its
/// help. Every other tool does nothing useful bare, so its tile types the name
/// and leaves the cursor after it for the arguments: grex stops on a missing
/// argument; jsonxf, huniq and csview read stdin when given no file and would
/// sit waiting on the keyboard; and tokei and dotenv-linter work on the
/// current directory, which for a tool is always `/` (nonos-std getcwd), so
/// tokei would count the whole file system. A tool added to apps.list is
/// typed, not run, until it is named here.
const RUNS_BARE: [&[u8]; 1] = [b"pastel"];

/// A command waiting for the Terminal to ask.
pub struct PendingCommand {
    pub line: String,
    /// When it was left, on uptime: the wall clock reads an error before the
    /// RTC is read and NTP steps it back, which made a command left then
    /// stale at once, or kept it for hours.
    pub at_ms: i64,
}

/// Whether `arg` may be left for an app as a path.
pub fn is_path(arg: &str) -> bool {
    arg.starts_with('/')
}

pub fn runs_bare(tool: &[u8]) -> bool {
    RUNS_BARE.contains(&tool)
}

/// What a tool tile hands the Terminal, or None for a name that is not a
/// plain tool name (the generated table only holds such names).
pub fn tool_command(tool: &[u8]) -> Option<String> {
    let name = core::str::from_utf8(tool).ok()?;
    let plain = name.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_');
    if name.is_empty() || !plain {
        return None;
    }
    let mut line = String::with_capacity(TYPE.len() + name.len() + 1);
    if runs_bare(tool) {
        line.push_str(RUN);
        line.push_str(name);
    } else {
        line.push_str(TYPE);
        line.push_str(name);
        line.push(' ');
    }
    Some(line)
}

/// Whether a command left at `at_ms` may still be handed over at `now`.
pub fn still_wanted(at_ms: i64, now: i64) -> bool {
    (0..COMMAND_TTL_MS).contains(&now.saturating_sub(at_ms))
}
