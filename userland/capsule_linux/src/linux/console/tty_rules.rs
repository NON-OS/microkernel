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

//! The tty rules the console applies, pure so the host proofs hold them:
//! when the NONOS terminal is told a guest left canonical mode or came back
//! (?7727), when a typed byte is Ctrl+C for a tty that raises signals, and
//! the line that says where it went.

use super::queue::Queue;
use super::queue_piece::Piece;

/// c_lflag's offset in the kernel termios, and ISIG and ICANON in it.
const LFLAG_AT: usize = 12;
const ISIG: u8 = 0o1;
const ICANON: u8 = 0o2;
/// VINTR's offset: c_cc[0] after c_line.
const VINTR_AT: usize = 17;

/// The sequence that tells the terminal of a change of canonical mode.
pub fn mode_switch(was: &[u8], now: &[u8]) -> Option<&'static [u8]> {
    let canon = |t: &[u8]| t.get(LFLAG_AT).is_some_and(|b| b & ICANON != 0);
    match (canon(was), canon(now)) {
        (true, false) => Some(b"\x1b[?7727h"),
        (false, true) => Some(b"\x1b[?7727l"),
        _ => None,
    }
}

/// VINTR when the settings raise signals and name one, and stdin is the
/// terminal (`tty`): n_tty applies ISIG to a tty only, so input given from
/// a file (`< f`) is read whole whatever bytes it holds.
pub fn interrupt_byte(t: &[u8], tty: bool) -> Option<u8> {
    let isig = tty && t.get(LFLAG_AT).is_some_and(|b| b & ISIG != 0);
    t.get(VINTR_AT).copied().filter(|&b| isig && b != 0)
}

/// Whether anything unread holds `byte`.
pub fn holds(queue: &Queue, byte: u8) -> bool {
    queue.pieces.iter().any(|p| matches!(p, Piece::Bytes(b, at) if b[*at..].contains(&byte)))
}

/// The log line for one Ctrl+C: the group, in the guests' numbering, and
/// how many of its processes took the SIGINT. 0 says nobody did.
pub fn interrupt_line(group: u32, took: usize) -> alloc::string::String {
    alloc::format!("[LINUX] Ctrl+C: SIGINT to group {group}, {took} processes\n")
}
