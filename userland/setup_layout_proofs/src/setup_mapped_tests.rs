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

//! Setup's reading of a map's answer and of the display it is given
//! (capsule_setup_wizard/src/setup/mapped.rs). The errnos are the kernel's
//! own (src/syscall/microkernel/errnos.rs), each of which sys_mmap returns
//! for a map it refuses.

use crate::setup_mapped::{mapped, shape_ok};

const ERRNO_PERM: i64 = -1;
const ERRNO_NOMEM: i64 = -12;
const ERRNO_INVAL: i64 = -22;

fn answer(rc: i64) -> *mut u8 {
    rc as *mut u8
}

#[test]
fn every_refused_map_is_no_address() {
    for rc in [ERRNO_PERM, ERRNO_NOMEM, ERRNO_INVAL, i64::MIN, 0] {
        assert_eq!(mapped(answer(rc)), None, "rc {rc}");
    }
}

#[test]
fn a_refused_map_is_not_null() {
    // Why a null check alone took an error for an address: the frame setup
    // drew in was then the errno itself, and the first draw faulted.
    assert!(!answer(ERRNO_NOMEM).is_null());
}

#[test]
fn a_mapped_address_is_kept() {
    for at in [0x1000u64, 0x4000_0000, 0x7FFF_FFFF_F000] {
        assert_eq!(mapped(at as *mut u8), Some(at));
    }
}

#[test]
fn a_stride_must_hold_a_row() {
    assert!(shape_ok(1920, 1080, 1920 * 4));
    assert!(shape_ok(1366, 768, 1376 * 4));
    assert!(!shape_ok(1920, 1080, 1920 * 4 - 1));
    assert!(!shape_ok(1920, 1080, 3));
    assert!(!shape_ok(1, 1, 0));
}

#[test]
fn no_side_of_nothing() {
    assert!(!shape_ok(0, 1080, 7680));
    assert!(!shape_ok(1920, 0, 7680));
}
