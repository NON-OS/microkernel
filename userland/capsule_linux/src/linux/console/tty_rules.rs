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
/// ECHO in c_lflag: off while a program reads a password.
const ECHO: u8 = 0o10;
/// VINTR's offset: c_cc[0] after c_line.
const VINTR_AT: usize = 17;
/// c_iflag's ISTRIP, INLCR, IGNCR and ICRNL: the input mapping n_tty does
/// before a byte is read, raw or in a line.
const ISTRIP: u32 = 0o40;
const INLCR: u32 = 0o100;
const IGNCR: u32 = 0o200;
const ICRNL: u32 = 0o400;

/// The sequence that tells the terminal of a change of canonical mode.
pub fn mode_switch(was: &[u8], now: &[u8]) -> Option<&'static [u8]> {
    let canon = |t: &[u8]| t.get(LFLAG_AT).is_some_and(|b| b & ICANON != 0);
    match (canon(was), canon(now)) {
        (true, false) => Some(b"\x1b[?7727h"),
        (false, true) => Some(b"\x1b[?7727l"),
        _ => None,
    }
}

/// The sequence that tells the terminal its line editor must stop showing
/// what is typed, or show it again: a password prompt turns ECHO off and
/// stays canonical, so ?7727 alone left the typed password on the screen.
pub fn echo_switch(was: &[u8], now: &[u8]) -> Option<&'static [u8]> {
    let echo = |t: &[u8]| t.get(LFLAG_AT).is_some_and(|b| b & ECHO != 0);
    match (echo(was), echo(now)) {
        (true, false) => Some(b"\x1b[?7728h"),
        (false, true) => Some(b"\x1b[?7728l"),
        _ => None,
    }
}

/// `got`, typed, mapped in place as c_iflag says, as Linux's n_tty maps
/// what a terminal sends: the eighth bit stripped under ISTRIP, a carriage
/// return dropped under IGNCR or made a newline under ICRNL, a newline made
/// a carriage return under INLCR. How many bytes are left at its start.
/// The NONOS terminal sends Enter as a carriage return once a program reads
/// raw keys (?7727), and a program that keeps ICRNL, as most raw-mode
/// programs do, waits for the newline it makes.
pub fn map_input(t: &[u8], got: &mut [u8]) -> usize {
    let i = t.get(..4).map_or(0, |w| u32::from_le_bytes([w[0], w[1], w[2], w[3]]));
    let mut kept = 0;
    for at in 0..got.len() {
        let c = if i & ISTRIP != 0 { got[at] & 0x7f } else { got[at] };
        let c = match c {
            b'\r' if i & IGNCR != 0 => continue,
            b'\r' if i & ICRNL != 0 => b'\n',
            b'\n' if i & INLCR != 0 => b'\r',
            c => c,
        };
        got[kept] = c;
        kept += 1;
    }
    kept
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
/// how many of its processes took the signal. 0 says nobody did.
pub fn interrupt_line(signal: &str, group: u32, took: usize) -> alloc::string::String {
    alloc::format!("[LINUX] Ctrl+C: {signal} to group {group}, {took} processes\n")
}
