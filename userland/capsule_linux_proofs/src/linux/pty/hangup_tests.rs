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

//! The ends of a pair: when foot (the master) goes, the shell reads end of
//! file and its session leader gets SIGHUP; when the shell goes, foot reads
//! EIO and poll says POLLHUP; a pair nothing names is let go.

use super::note::{SIGCONT, SIGHUP};
use super::support::{read, table, EAGAIN, EIO};
use super::table::{Table, To};

const POLLHUP: u16 = 0x010;
const LEADER: u32 = 40;

#[test]
fn the_master_closing_hangs_the_slave_up() {
    let mut t = table(3);
    t.get(0).unwrap().session = LEADER;
    t.typed_at(0, b"unread").unwrap();
    t.note([(0, false)].into_iter());
    assert_eq!(t.signals, [(To::Leader(LEADER), SIGHUP), (To::Leader(LEADER), SIGCONT)]);
    let p = t.get(0).unwrap();
    assert!(p.hung && p.session == 0, "and it controls no session after");
    assert_eq!(read(p).unwrap(), b"", "the shell reads end of file");
    assert_eq!(p.slave_write(b"x"), Err(EIO));
    assert_ne!(p.bits(false) & POLLHUP, 0);
    assert_eq!(t.may_open(0), Err(2), "and /dev/pts/0 is gone");
    assert!(t.names().is_empty());
}

#[test]
fn the_slave_closing_is_eio_for_the_master_after_its_output() {
    let mut t = table(0);
    t.get(0).unwrap().slave_write(b"bye\n").unwrap();
    t.note([(0, true)].into_iter());
    let p = t.get(0).unwrap();
    assert_ne!(p.bits(true) & POLLHUP, 0, "foot's epoll sees the shell gone");
    assert_eq!(p.master_read(64).unwrap(), b"bye\r\n", "what it wrote last is still read");
    assert_eq!(p.master_read(64), Err(EIO));
    assert_eq!(p.master_write(b"x"), Err(EIO));
}

#[test]
fn before_any_slave_opens_the_master_waits() {
    let mut t = Table::new();
    let n = t.make().unwrap();
    let p = t.get(n).unwrap();
    assert_eq!(p.master_read(64), Err(EAGAIN));
    assert_eq!(p.bits(true) & POLLHUP, 0, "foot's child has not opened it yet");
}

#[test]
fn a_pair_nothing_names_is_let_go_and_its_number_used_again() {
    let mut t = table(0);
    assert_eq!(t.make(), Some(1));
    t.note([(1, true), (1, false)].into_iter());
    assert!(t.pairs[0].is_none(), "pair 0 had no descriptor left");
    assert_eq!(t.make(), Some(0));
}
