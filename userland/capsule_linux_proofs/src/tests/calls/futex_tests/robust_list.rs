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

//! A robust mutex's probe, read where it is answered: get_robust_list is
//! served, a thread's own list reads as none and 24 bytes long, and another
//! thread's is refused. musl asks this before it makes a mutex robust, and
//! LMDB's lock file, the Monero wallet's ring database, failed without it
//! ("[LINUX] unserved nr=274").

const ALL: &str = include_str!("../../../../../capsule_linux/src/linux/serve/table/all.rs");
const NR: &str = include_str!("../../../../../capsule_linux/src/linux/abi/nr_high.rs");
const ROBUST: &str = include_str!("../../../../../capsule_linux/src/linux/call/robust.rs");

#[test]
fn the_probe_is_served_and_answers_none_for_the_caller_alone() {
    assert!(NR.contains("pub const GET_ROBUST_LIST: u64 = 274;"));
    assert!(ALL.contains("nr::GET_ROBUST_LIST => call::get_robust_list(guest, a[0], a[1], a[2]),"));
    let refuse = ROBUST.find("if pid != 0 {").expect("another thread's list");
    assert!(
        ROBUST[refuse..].starts_with("if pid != 0 {\n        return errno::fail(errno::EPERM);")
    );
    assert!(ROBUST.contains("const HEAD_BYTES: u64 = 24;"));
    let none = ROBUST.find("guest.write(head, &0u64.to_le_bytes()) == 8").expect("no head");
    let size = ROBUST.find("guest.write(len, &HEAD_BYTES.to_le_bytes()) == 8").expect("its size");
    assert!(refuse < none && none < size, "the refusal comes before anything is written");
}
