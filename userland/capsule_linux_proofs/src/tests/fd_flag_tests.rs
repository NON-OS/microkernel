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

//! The flags of the calls that make a descriptor, against Linux's checks
//! (fs/file.c ksys_dup3, fs/eventpoll.c do_epoll_create, mm/memfd.c): what
//! is refused, with its errno, and whether the descriptor made is close-on-exec.

use crate::linux::abi::errno::EINVAL;
use crate::linux::file::create_flags::{epoll_flags, memfd_flags};
use crate::linux::file::dup3_args::dup3_args;

const O_CLOEXEC: u64 = 0o2000000;

#[test]
fn dup3_takes_o_cloexec_and_nothing_else() {
    assert_eq!(dup3_args(3, 7, 0), Ok(false));
    assert_eq!(dup3_args(3, 7, O_CLOEXEC), Ok(true));
    for bad in [1, 0o4000, 0o200000, O_CLOEXEC | 0o4000, 1 << 31] {
        assert_eq!(dup3_args(3, 7, bad), Err(EINVAL), "{bad:#o}");
    }
}

#[test]
fn dup3_onto_itself_is_einval_whatever_the_flags() {
    assert_eq!(dup3_args(5, 5, 0), Err(EINVAL));
    assert_eq!(dup3_args(5, 5, O_CLOEXEC), Err(EINVAL));
    // The numbers are unsigned ints: only their low halves are compared.
    assert_eq!(dup3_args(5, (1 << 32) | 5, 0), Err(EINVAL));
}

#[test]
fn dup3_reads_its_flags_as_an_int() {
    // The upper half of the register is not part of the argument.
    assert_eq!(dup3_args(3, 7, (1 << 40) | O_CLOEXEC), Ok(true));
    assert_eq!(dup3_args(3, 7, 1 << 40), Ok(false));
}

#[test]
fn epoll_create1_takes_epoll_cloexec_and_nothing_else() {
    assert_eq!(epoll_flags(0), Ok(false));
    assert_eq!(epoll_flags(O_CLOEXEC), Ok(true));
    // EPOLL_NONBLOCK does not exist; O_NONBLOCK is refused like any other.
    for bad in [1, 0o4000, O_CLOEXEC | 0o4000, 1 << 31] {
        assert_eq!(epoll_flags(bad), Err(EINVAL), "{bad:#o}");
    }
    assert_eq!(epoll_flags((1 << 40) | O_CLOEXEC), Ok(true));
}

#[test]
fn memfd_create_takes_its_five_flags_and_a_huge_size_only_with_hugetlb() {
    assert_eq!(memfd_flags(0), Ok(false));
    // Python's os.memfd_create default, and with sealing allowed.
    assert_eq!(memfd_flags(0x1), Ok(true));
    assert_eq!(memfd_flags(0x1 | 0x2), Ok(true));
    // MFD_HUGETLB with MFD_HUGE_2MB (21 << 26).
    assert_eq!(memfd_flags(0x4 | (21 << 26)), Ok(false));
    assert_eq!(memfd_flags(21 << 26), Err(EINVAL));
    assert_eq!(memfd_flags(0x20), Err(EINVAL));
    // MFD_EXEC with MFD_NOEXEC_SEAL asks for opposite things.
    assert_eq!(memfd_flags(0x8 | 0x10), Err(EINVAL));
    assert_eq!(memfd_flags((1 << 40) | 0x1), Ok(true));
}
