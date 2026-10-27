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

//! A master that writes and never reads: whatever it types must stop at a
//! bound, or one write loop grows the personality until it runs out of
//! memory. Each ^C is echoed onto the master's own output, each ^D on an
//! empty line is an end of file kept for the slave, and each signal waits
//! for the serve loop.

use super::disc::{SIGINT, SIGQUIT};
use super::flags::{lflag, ECHO};
use super::input::MOST;
use super::out::OUT_MOST;
use super::support::{table, EAGAIN};
use super::table::To;

const SHELL: u32 = 7;

#[test]
fn a_flood_of_ctrl_c_stops_at_the_masters_room() {
    let mut t = table(SHELL);
    let flood = alloc::vec![0x03u8; 64 << 10];
    let refused = (0..8).any(|_| t.typed_at(0, &flood) == Err(EAGAIN));
    assert!(refused, "a master that is never read stops taking bytes");
    let held = t.get(0).unwrap().queued(true);
    assert!(held <= OUT_MOST + 8, "{held} bytes held for the master");
}

/// ^D on an empty line ends a read with nothing and adds no byte, so a
/// master that types only ^D must still find the slave's buffer full.
#[test]
fn a_flood_of_ctrl_d_stops_at_the_slaves_room() {
    let mut t = table(SHELL);
    let flood = alloc::vec![0x04u8; 64 << 10];
    let refused = (0..8).any(|_| t.typed_at(0, &flood) == Err(EAGAIN));
    assert!(refused, "a slave that is never read stops taking ends of file");
    assert!(t.get(0).unwrap().input.ends.len() <= MOST);
}

/// With echo off nothing fills the master, so each ^C must not be kept:
/// one of each signal waits for the serve loop, as a pending set holds it.
#[test]
fn a_flood_of_ctrl_c_with_echo_off_raises_one_signal() {
    let mut t = table(SHELL);
    let p = t.get(0).unwrap();
    let l = lflag(&p.termios) & !ECHO;
    p.termios[12..16].copy_from_slice(&l.to_le_bytes());
    let flood = alloc::vec![0x03u8, 0x1c].repeat(32 << 10);
    assert_eq!(t.typed_at(0, &flood), Ok(64 << 10));
    assert_eq!(t.signals, [(To::Group(SHELL), SIGINT), (To::Group(SHELL), SIGQUIT)]);
}
