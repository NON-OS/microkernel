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

//! ISIG: ^C typed at foot's master is SIGINT for the slave's foreground
//! group, not a byte; and a new window size is SIGWINCH for it.

use super::disc::{SIGINT, SIGQUIT, SIGTSTP};
use super::flags::ISIG;
use super::support::{read, table, without, EAGAIN};
use super::table::To;
use super::table_io::SIGWINCH;

const SHELL: u32 = 7;

#[test]
fn ctrl_c_is_sigint_for_the_foreground_group_and_drops_the_line() {
    let mut t = table(SHELL);
    assert_eq!(t.typed_at(0, b"sleep 100"), Ok(9));
    assert_eq!(t.typed_at(0, b"\x03"), Ok(1));
    assert_eq!(t.signals, [(To::Group(SHELL), SIGINT)]);
    let p = t.get(0).unwrap();
    assert!(p.master_read(64).unwrap().ends_with(b"^C"), "echoed as ^C");
    assert_eq!(read(p), Err(EAGAIN), "what was typed before it is gone");
}

#[test]
fn quit_and_suspend_raise_their_own_signals() {
    let mut t = table(SHELL);
    t.typed_at(0, b"\x1c\x1a").unwrap();
    assert_eq!(t.signals, [(To::Group(SHELL), SIGQUIT), (To::Group(SHELL), SIGTSTP)]);
    let mut none = table(0);
    none.typed_at(0, b"\x03").unwrap();
    assert!(none.signals.is_empty(), "with no foreground group nobody is signalled");
}

#[test]
fn without_isig_ctrl_c_is_a_byte_the_program_reads() {
    let mut t = table(SHELL);
    let p = t.get(0).unwrap();
    p.set_termios(without(p.termios, 12, ISIG));
    t.typed_at(0, b"\x03\n").unwrap();
    assert!(t.signals.is_empty());
    assert_eq!(read(t.get(0).unwrap()).unwrap(), b"\x03\n");
}

#[test]
fn a_new_window_size_is_sigwinch_only_when_it_changes() {
    let mut t = table(SHELL);
    let ws = |rows: u16, cols: u16| {
        let mut w = [0u8; 8];
        w[..2].copy_from_slice(&rows.to_le_bytes());
        w[2..4].copy_from_slice(&cols.to_le_bytes());
        w
    };
    assert_eq!(t.resize(0, ws(24, 80)), Ok(()));
    assert_eq!(t.resize(0, ws(24, 80)), Ok(()));
    assert_eq!(t.signals, [(To::Group(SHELL), SIGWINCH)], "the same size again is quiet");
    assert_eq!(t.get(0).unwrap().winsize, ws(24, 80), "TIOCGWINSZ gives it back");
    assert_eq!(t.resize(9, ws(1, 1)), Err(5), "no such pair");
}
