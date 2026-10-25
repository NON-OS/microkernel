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

//! The frame a supervisor reads, held to the numbers and the fields
//! nonos_libc gives the same names.

use core::mem::{offset_of, size_of};

use crate::foreign::frame::{ForeignFrame, NR_DIED, NR_INTERRUPTED};

const LIBC: &str = include_str!("../../../../libc/src/foreign_frame.rs");
const KERNEL: &str = include_str!("../../../../../src/process/foreign/frame.rs");

/// The `pub name: type` lines of the one struct in `src`, in order.
fn fields(src: &str) -> Vec<&str> {
    src.lines().map(str::trim).filter(|l| l.starts_with("pub ") && l.ends_with(',')).collect()
}

#[test]
fn the_two_kernel_numbers_are_the_libc_s() {
    assert!(LIBC.contains("pub const FOREIGN_NR_DIED: u64 = u64::MAX;"));
    assert!(LIBC.contains("pub const FOREIGN_NR_INTERRUPTED: u64 = u64::MAX - 1;"));
    assert_eq!((NR_DIED, NR_INTERRUPTED), (u64::MAX, u64::MAX - 1));
}

#[test]
fn a_frame_carries_each_argument_in_its_wire_slot() {
    let f = ForeignFrame::new(7, 39, [1, 2, 3, 4, 5, 6], 0x1000);
    assert_eq!((f.pid, f._pad, f.nr, f.rip), (7, 0, 39, 0x1000));
    assert_eq!([f.arg0, f.arg1, f.arg2, f.arg3, f.arg4, f.arg5], [1, 2, 3, 4, 5, 6]);
    assert_eq!((size_of::<ForeignFrame>(), offset_of!(ForeignFrame, nr)), (72, 8));
    assert_eq!((offset_of!(ForeignFrame, arg0), offset_of!(ForeignFrame, rip)), (16, 64));
    assert_eq!(fields(KERNEL), fields(LIBC), "nonos_libc's frame, field for field");
    assert_eq!(fields(KERNEL).len(), 10);
}
