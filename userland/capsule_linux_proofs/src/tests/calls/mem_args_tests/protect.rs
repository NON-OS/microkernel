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

//! mmap's checks before its policy, its hint, and mprotect's arguments.

use super::mmap::{ProtRow, ANON, GROWSDOWN, GROWSUP, PRIVATE, RW, RWX, RX};
use crate::linux::abi::errno::{EINVAL, ENOMEM, EPERM};
use crate::linux::call::mem::map_args::{flags::MIN_ADDR, hint, map_args};
use crate::linux::call::mem::prot_args::prot_span;
use crate::linux::guest::{PAGE, USER_MAX};

#[test]
fn a_hint_is_rounded_down_and_never_below_mmap_min_addr() {
    assert_eq!(hint(0), None);
    assert_eq!(hint(PAGE - 1), None, "inside page zero is no hint at all");
    assert_eq!(hint(PAGE), Some(MIN_ADDR));
    assert_eq!(hint(MIN_ADDR - 1), Some(MIN_ADDR));
    assert_eq!(hint(0x1234_5678), Some(0x1234_5000));
    assert_eq!(hint(u64::MAX), Some(!(PAGE - 1)));
}

#[test]
fn mprotect_refuses_each_bad_argument_with_linux_errno() {
    let at = 0x10_0000;
    let table: [ProtRow; 14] = [
        (at + 1, PAGE, 1, Err(EINVAL)),
        (at, PAGE, GROWSDOWN | GROWSUP | 1, Err(EINVAL)),
        // An empty span succeeds before its protection is looked at.
        (at, 0, 0xff, Ok(None)),
        // An end that wraps is ENOMEM, as Linux's `end <= start` has it.
        (at, u64::MAX, 1, Err(ENOMEM)),
        (at, u64::MAX - at, 1, Err(ENOMEM)),
        (at, PAGE, 0x10, Err(EINVAL)),
        (at, PAGE, 1 << 40, Err(EINVAL)),
        (at, PAGE, GROWSDOWN | 1, Err(EINVAL)),
        (at, PAGE, GROWSUP, Err(EINVAL)),
        (at, PAGE, RWX, Err(EPERM)),
        (USER_MAX, PAGE, 1, Err(ENOMEM)),
        (USER_MAX - PAGE, PAGE + 1, 1, Err(ENOMEM)),
        // PROT_SEM is accepted, as x86 accepts it.
        (at, 1, 8 | 1, Ok(Some((at, PAGE)))),
        (USER_MAX - PAGE, PAGE, RX, Ok(Some((USER_MAX - PAGE, PAGE)))),
    ];
    for (addr, len, prot, want) in table {
        assert_eq!(prot_span(addr, len, prot), want, "{addr:#x} {len:#x} {prot:#x}");
    }
}

#[test]
fn mmap_checks_its_arguments_before_its_own_policy() {
    // A misaligned offset is EINVAL even when the protection is refused too.
    assert_eq!(map_args(0, PAGE, RWX, PRIVATE | ANON, 3), Err(EINVAL));
    // A hint, unlike an exact address, may be anywhere at all.
    assert_eq!(map_args(1, PAGE, RW, PRIVATE | ANON, 0), Ok(()));
}
