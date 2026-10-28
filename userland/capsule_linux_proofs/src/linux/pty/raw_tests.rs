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

//! The discipline out of canonical mode, as a line editor puts it, with
//! echo off, as a password prompt puts it, and output processing.

use super::flags::{ECHO, ICANON, ONLCR, VMIN};
use super::support::{pair, read, type_in, without, EAGAIN};

#[test]
fn raw_mode_gives_each_byte_at_once() {
    let mut p = pair();
    type_in(&mut p, b"half");
    p.set_termios(without(p.termios, 12, ICANON));
    assert_eq!(read(&mut p).unwrap(), b"half", "leaving ICANON hands over the line");
    type_in(&mut p, b"\x1b[A\x7f");
    assert_eq!(read(&mut p).unwrap(), b"\x1b[A\x7f", "no editing, nothing held");
    assert_eq!(read(&mut p), Err(EAGAIN), "VMIN 1 waits for a byte");
    p.termios[17 + VMIN] = 0;
    assert_eq!(read(&mut p).unwrap(), b"", "VMIN 0 and VTIME 0 never wait");
}

#[test]
fn echo_off_is_silent_and_output_gets_cr_lf() {
    let mut p = pair();
    p.set_termios(without(p.termios, 12, ECHO));
    assert_eq!(type_in(&mut p, b"secret\n"), b"", "a password prompt echoes nothing");
    assert_eq!(read(&mut p).unwrap(), b"secret\n");
    p.set_termios(without(p.termios, 12, 0));
    assert_eq!(p.slave_write(b"$ ok\n"), Ok(5));
    assert_eq!(p.master_read(64).unwrap(), b"$ ok\r\n");
    p.set_termios(without(p.termios, 4, ONLCR));
    p.slave_write(b"raw\n").unwrap();
    assert_eq!(p.master_read(64).unwrap(), b"raw\n");
}

#[test]
fn a_control_byte_is_echoed_as_a_caret() {
    let mut p = pair();
    assert_eq!(type_in(&mut p, b"\x01\t"), b"^A\t", "ECHOCTL, but a tab is a tab");
    assert_eq!(type_in(&mut p, b"\x7f\x7f"), b"\x08 \x08\x08 \x08\x08 \x08", "^A takes two");
}
