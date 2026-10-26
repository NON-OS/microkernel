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

//! A Linux package's pack, held open by the run reading it, is never
//! replaced or removed by another process: unlinking it or renaming over it
//! is refused, a new handle on it cannot write, and once the run's handles
//! close, or the run ends and is reaped, the install may replace it.

use crate::vfs_store::Store;

const RUN: u32 = 40;
const INSTALL: u32 = 41;
const PACK: &str = "/linux-packs/vim.pack";

fn put(s: &mut Store, path: &str, pid: u32, bytes: &[u8]) {
    let fd = s.open(path, pid, true, true, false, true).expect("opens");
    s.write(fd, pid, bytes).expect("writes");
    s.close(fd, pid).expect("closes");
}

#[test]
fn a_held_pack_is_neither_replaced_nor_removed_nor_written() {
    let mut s = Store::new();
    put(&mut s, PACK, INSTALL, b"v1");
    let held = s.open(PACK, RUN, false, false, false, false).expect("the run opens it");
    assert!(s.held_by_another(PACK, INSTALL));
    assert!(!s.held_by_another(PACK, RUN), "its own hold does not stop the run");
    put(&mut s, "/linux-packs/vim.pack.new", INSTALL, b"v2");
    assert!(s.held_by_another(PACK, INSTALL), "the rename the install would make is held");
    let w = s.open(PACK, INSTALL, false, false, false, true).expect("opens to read");
    assert!(s.write(w, INSTALL, b"xx").is_err(), "a handle on a held pack cannot write");
    assert_eq!(s.read(held, RUN, 8).expect("reads"), b"v1");
    s.close(held, RUN).expect("the run ends");
    s.close(w, INSTALL).expect("closes");
    assert!(!s.held_by_another(PACK, INSTALL), "nothing holds it now");
}

#[test]
fn a_crashed_runs_hold_goes_when_it_is_reaped() {
    let mut s = Store::new();
    put(&mut s, PACK, INSTALL, b"v1");
    s.open(PACK, RUN, false, false, false, false).expect("the run opens it");
    assert!(s.held_by_another(PACK, INSTALL));
    s.close_ended(|pid| pid != RUN);
    assert!(!s.held_by_another(PACK, INSTALL));
}

#[test]
fn the_hold_is_the_packs_alone() {
    let mut s = Store::new();
    put(&mut s, "/home/nonos/a.txt", INSTALL, b"a");
    s.open("/home/nonos/a.txt", RUN, false, false, false, false).expect("opens");
    assert!(!s.held_by_another("/home/nonos/a.txt", INSTALL), "any other file is as it was");
}
