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

//! A command line in the shell's answer to OP_TAKE_OPEN_ARG. The editor,
//! Files and the players take a path there, which always starts with '/'. A
//! command for the Terminal starts with a word that no path can: `run:` to run
//! it as typed, `type:` to leave it on the prompt for the person to finish.
//! Anything else (a path, an empty answer, a line with a control byte in it)
//! is not a command, and the Terminal runs nothing. Kept apart from the IPC
//! so it can be proven (terminal_line_proofs handed_tests).

use crate::term::dimensions::LINE_MAX;

/// Hand-synced with desktop_shell's state::open_arg::{RUN, TYPE}.
pub const RUN: &[u8] = b"run:";
pub const TYPE: &[u8] = b"type:";

/// What the shell handed over, and whether it runs.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Handed<'a> {
    /// Echo the line as if typed and run it.
    Run(&'a [u8]),
    /// Put the line on the prompt, cursor at its end, and run nothing.
    Type(&'a [u8]),
}

/// `body` is the reply after the wire header: a 4-byte status, then the
/// argument. A bare status means the shell holds nothing for this window.
pub fn parse(body: &[u8]) -> Option<Handed<'_>> {
    let arg = body.get(4..)?;
    if let Some(line) = arg.strip_prefix(RUN) {
        return typed(line).filter(|l| l.iter().any(|b| *b != b' ')).map(Handed::Run);
    }
    arg.strip_prefix(TYPE).and_then(typed).map(Handed::Type)
}

/// The line as the keyboard could have typed it: UTF-8, no control byte (a
/// newline or an escape would end the line or drive the screen), and short
/// enough for the prompt, so what is shown is the whole of what runs.
fn typed(line: &[u8]) -> Option<&[u8]> {
    let ok = !line.is_empty()
        && line.len() <= LINE_MAX
        && core::str::from_utf8(line).is_ok()
        && !line.iter().any(|b| *b < 0x20 || *b == 0x7f);
    ok.then_some(line)
}
