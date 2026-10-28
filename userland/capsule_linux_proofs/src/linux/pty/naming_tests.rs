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

//! Finding a pair as musl does: posix_openpt's /dev/ptmx takes the lowest
//! free number, ptsname makes /dev/pts/N of TIOCGPTN's number, and that
//! slave opens only once unlockpt has cleared the lock.

use super::name::{handle, of_handle, pts_number, pts_path};
use super::support::{EIO, ENOENT};
use super::table::{Table, MAX};

#[test]
fn numbers_are_the_lowest_free() {
    let mut t = Table::new();
    assert_eq!([t.make(), t.make(), t.make()], [Some(0), Some(1), Some(2)]);
    t.pairs[1] = None;
    assert_eq!(t.make(), Some(1));
    while t.make().is_some() {}
    assert_eq!(t.pairs.len(), MAX, "ENOSPC past the most a family has");
}

#[test]
fn ptsname_and_the_path_name_the_same_pair() {
    for n in [0, 1, 9, 10, 63] {
        assert_eq!(pts_number(&pts_path(n)), Some(n));
        assert_eq!(of_handle(handle(n, true)), (n, true));
        assert_eq!(of_handle(handle(n, false)), (n, false));
    }
    for bad in [&b"/dev/pts/"[..], b"/dev/pts/01", b"/dev/pts/x", b"/dev/pts/-1", b"/dev/ptmx"] {
        assert_eq!(pts_number(bad), None);
    }
}

#[test]
fn the_slave_opens_only_after_unlockpt() {
    let mut t = Table::new();
    let n = t.make().unwrap();
    assert_eq!(t.may_open(n), Err(EIO), "locked, as posix_openpt leaves it");
    assert_eq!(t.names(), [n], "though /dev/pts lists it");
    t.get(n).unwrap().locked = false;
    assert_eq!(t.may_open(n), Ok(()));
    assert_eq!(t.may_open(n + 1), Err(ENOENT));
}

#[test]
fn a_session_finds_the_terminal_it_controls() {
    let mut t = Table::new();
    let n = t.make().unwrap();
    assert_eq!(t.controlling(40), None);
    t.get(n).unwrap().session = 40;
    assert_eq!(t.controlling(40), Some(n), "/dev/tty opens it");
    assert_eq!(t.controlling(0), None, "no session is nobody's");
}
