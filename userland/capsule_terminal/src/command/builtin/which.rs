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

//! `type` (and `which`): say what a name will actually run.
//!
//! This shell resolves a name through four routes that look identical at the
//! prompt. `ls` is code inside the terminal. `rg` is a separate signed capsule
//! the kernel spawns and streams back. `sd` is loaded from the package store.
//! `uptime` is handled by the chain command set. Typing them feels the same
//! and what happens underneath does not, and the difference is exactly the
//! sort of thing this system asks people to be able to check.
//!
//! The order below is the order the dispatcher really uses, so the answer is
//! what would run rather than what exists somewhere.

use crate::command::builtin::tool;
use crate::command::output::Output;
use crate::event::complete::is_command_name;
use crate::jobs::is_store_tool;

pub fn run(out: &mut Output<'_>, argv: &[&[u8]]) -> bool {
    if argv.len() < 2 {
        out.writeln(b"usage: type <name> [name ...]");
        return false;
    }
    let mut all_found = true;
    for name in &argv[1..] {
        all_found &= describe(out, name);
    }
    all_found
}

fn describe(out: &mut Output<'_>, name: &[u8]) -> bool {
    let mut line = [b' '; 120];
    let mut n = copy(&mut line, 0, name);

    if let Some(service) = tool::service_for(name) {
        n = copy(&mut line, n, b" is an installed tool, spawned as capsule tool.");
        n = copy(&mut line, n, service);
        out.writeln(&line[..n]);
        return true;
    }
    if is_store_tool(name) {
        n = copy(&mut line, n, b" is a tool loaded from the package store");
        out.writeln(&line[..n]);
        return true;
    }
    if is_command_name(name) {
        n = copy(&mut line, n, b" is built into the shell");
        out.writeln(&line[..n]);
        return true;
    }
    n = copy(&mut line, n, b" not found");
    out.writeln(&line[..n]);
    false
}

/// Append into a fixed line, stopping at its end rather than wrapping. A name
/// long enough to reach the edge is already unreadable; truncating it beats
/// pushing the explanation off the row.
fn copy(line: &mut [u8], at: usize, text: &[u8]) -> usize {
    let room = line.len().saturating_sub(at);
    let n = text.len().min(room);
    line[at..at + n].copy_from_slice(&text[..n]);
    at + n
}
