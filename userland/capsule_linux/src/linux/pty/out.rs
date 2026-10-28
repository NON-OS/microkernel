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

//! What reaches a pseudo-terminal's master side: the slave's writes after
//! output processing, and the discipline's echo of what was typed. Pure.

use alloc::collections::VecDeque;

use super::flags::{lflag, oflag, Termios, ECHO, ECHOCTL, ECHOE, ECHONL, ICANON};
use super::flags::{OCRNL, ONLCR, OPOST};

/// The most the master side holds unread; a slave writer waits past it.
pub const OUT_MOST: usize = 64 << 10;

/// `bytes` as OPOST has them: ONLCR makes each newline CR LF, which is what
/// lets a shell's plain "\n" start the terminal's next line at its left.
pub fn post(t: &Termios, bytes: &[u8], to: &mut VecDeque<u8>) {
    let o = oflag(t);
    for &b in bytes {
        match b {
            b'\n' if o & (OPOST | ONLCR) == OPOST | ONLCR => to.extend(*b"\r\n"),
            b'\r' if o & (OPOST | OCRNL) == OPOST | OCRNL => to.push_back(b'\n'),
            _ => to.push_back(b),
        }
    }
}

/// A control byte other than tab and newline, which ECHOCTL shows as ^X.
fn shown_as_caret(t: &Termios, b: u8) -> bool {
    lflag(t) & ECHOCTL != 0 && ((b < 0x20 && b != b'\t' && b != b'\n') || b == 0x7f)
}

/// The echo of one typed byte: under ECHO as it is (or ^X), and a newline
/// alone under ECHONL in canonical mode.
pub fn echo(t: &Termios, b: u8, to: &mut VecDeque<u8>) {
    let l = lflag(t);
    if l & ECHO == 0 {
        if b == b'\n' && l & (ECHONL | ICANON) == ECHONL | ICANON {
            post(t, b"\n", to);
        }
        return;
    }
    match shown_as_caret(t, b) {
        true => post(t, &[b'^', b ^ 0x40], to),
        false => post(t, &[b], to),
    }
}

/// Byte `b` taken off the line being edited, rubbed out on the screen
/// under ECHO and ECHOE: two columns for a ^X, else one.
pub fn rub(t: &Termios, b: u8, to: &mut VecDeque<u8>) {
    if lflag(t) & (ECHO | ECHOE) != ECHO | ECHOE {
        return;
    }
    for _ in 0..1 + usize::from(shown_as_caret(t, b)) {
        to.extend(*b"\x08 \x08");
    }
}
