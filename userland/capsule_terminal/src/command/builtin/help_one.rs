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

//! `help <command>`: what one command takes.
//!
//! The grouped list from bare `help` says a command exists. It cannot say what
//! `-r` does to `grep` or whether `head` takes a count, and until now nothing
//! could: the flags each command accepts were declared inside its own body,
//! reachable by reading the source and by no other means. A shell whose only
//! documentation is its source is a shell for the people who wrote it.
//!
//! Each entry is the usage line and one sentence. Where a command declares a
//! flag table this matches it; anything added there and not here shows up as a
//! flag the shell accepts and does not describe, which is the failure mode to
//! watch for.

use crate::command::output::Output;

use super::help_unknown::unknown;
use super::help_usage::table::USAGE;

/// Print one command's usage. Returns false when the name is unknown, so the
/// caller can set a failure status and `help x && y` behaves.
pub fn run(out: &mut Output<'_>, name: &[u8]) -> bool {
    let found = USAGE.iter().flat_map(|group| group.iter()).find(|(n, _, _)| *n == name);
    let Some((_, usage, what)) = found else {
        return unknown(out, name);
    };
    out.writeln(usage);
    let mut line = [b' '; 118];
    let n = 2 + what.len().min(line.len() - 2);
    line[2..n].copy_from_slice(&what[..n - 2]);
    out.writeln(&line[..n]);
    if name == b"qwen" {
        for more in super::qwen::HELP {
            out.writeln(more);
        }
    }
    true
}
