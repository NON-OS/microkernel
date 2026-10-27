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

//! Which opens under /dev/shm are a window buffer, made as a memfd: musl's
//! shm_open of a fresh name, read and write, as swayimg makes its buffers.

use crate::linux::file::shm_rule::is_buffer;

const O_RDONLY: u64 = 0;
const O_RDWR: u64 = 0o2;
const O_CREAT: u64 = 0o100;
const O_EXCL: u64 = 0o200;
// musl's shm_open adds these to what the caller asks.
const SHM_OPEN: u64 = 0o400000 | 0o4000 | 0o2000000;

#[test]
fn a_fresh_read_write_name_under_dev_shm_is_a_buffer() {
    let flags = O_RDWR | O_CREAT | O_EXCL | SHM_OPEN;
    assert!(is_buffer(b"/dev/shm/swayimg-1234", flags));
    assert!(is_buffer(b"/dev/shm/wl_shm-AbC09z", O_RDWR | O_CREAT | O_EXCL));
}

#[test]
fn an_open_that_could_meet_a_name_already_there_is_the_store_s() {
    assert!(!is_buffer(b"/dev/shm/x", O_RDWR | O_CREAT));
    assert!(!is_buffer(b"/dev/shm/x", O_RDWR));
    assert!(!is_buffer(b"/dev/shm/x", O_RDONLY | O_CREAT | O_EXCL));
}

#[test]
fn only_a_name_straight_under_dev_shm() {
    let flags = O_RDWR | O_CREAT | O_EXCL;
    assert!(!is_buffer(b"/dev/shm/", flags));
    assert!(!is_buffer(b"/dev/shm/a/b", flags));
    assert!(!is_buffer(b"/dev/shmx", flags));
    assert!(!is_buffer(b"/tmp/x", flags));
    assert!(!is_buffer(b"/root/dev/shm/x", flags));
}
