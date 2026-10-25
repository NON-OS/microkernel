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

//! mmap's arguments, each refused with Linux's errno.

use crate::linux::abi::errno::{EINVAL, ENOMEM, EPERM};
use crate::linux::call::mem::map_args::{flags::MIN_ADDR, map_args};
use crate::linux::guest::PAGE;

pub(super) const PRIVATE: u64 = 0x02;
const SHARED: u64 = 0x01;
const VALIDATE: u64 = 0x03;
pub(super) const ANON: u64 = 0x20;
pub(super) const FIXED: u64 = 0x10;
pub(super) const NOREPLACE: u64 = 0x10_0000;
pub(super) const RW: u64 = 3;
pub(super) const RX: u64 = 5;
pub(super) const RWX: u64 = 7;
pub(super) const GROWSDOWN: u64 = 0x0100_0000;
pub(super) const GROWSUP: u64 = 0x0200_0000;
pub(super) const MAYMOVE: u64 = 1;
pub(super) const MFIXED: u64 = 2;
pub(super) const DONTUNMAP: u64 = 4;

/// addr, len, prot, flags, off, and what mmap's rule answers.
type MapRow = (u64, u64, u64, u64, u64, Result<(), i64>);

/// addr, len, prot, and what mprotect's rule answers.
pub(super) type ProtRow = (u64, u64, u64, Result<Option<(u64, u64)>, i64>);

#[test]
fn mmap_refuses_each_bad_argument_with_linux_errno() {
    let table: [MapRow; 16] = [
        (0, 0, RW, PRIVATE | ANON, 0, Err(EINVAL)),
        (0, PAGE, RW, PRIVATE | ANON, 1, Err(EINVAL)),
        (0, PAGE, RW, PRIVATE | ANON, PAGE + 8, Err(EINVAL)),
        // Neither shared nor private, and a type Linux does not define.
        (0, PAGE, RW, ANON, 0, Err(EINVAL)),
        (0, PAGE, RW, 0x0f | ANON, 0, Err(EINVAL)),
        (0x10_0001, PAGE, RW, PRIVATE | ANON | FIXED, 0, Err(EINVAL)),
        (0x10_0001, PAGE, RW, PRIVATE | ANON | NOREPLACE, 0, Err(EINVAL)),
        (0, u64::MAX, RW, PRIVATE | ANON, 0, Err(ENOMEM)),
        (0, PAGE, RWX, PRIVATE | ANON, 0, Err(EPERM)),
        (0, PAGE, 6, PRIVATE | ANON, 0, Err(EPERM)),
        (0, PAGE, RW, PRIVATE | ANON | FIXED, 0, Err(EPERM)),
        (PAGE, PAGE, RW, PRIVATE | ANON | FIXED, 0, Err(EPERM)),
        (MIN_ADDR - PAGE, PAGE, RW, PRIVATE | ANON | NOREPLACE, 0, Err(EPERM)),
        (MIN_ADDR, PAGE, RW, PRIVATE | ANON | FIXED, 0, Ok(())),
        (0, PAGE, RX, SHARED | ANON, 0, Ok(())),
        (0, 1, 0, VALIDATE, PAGE, Ok(())),
    ];
    for (addr, len, prot, flags, off, want) in table {
        assert_eq!(map_args(addr, len, prot, flags, off), want, "{addr:#x} {len:#x} {flags:#x}");
    }
}
