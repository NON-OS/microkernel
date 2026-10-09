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

//! A buffer sized by a caller is taken fallibly, and the large copies take
//! theirs that way.
//!
//! The kernel's buffer rule is included by path. MkLocalSign and MkLocalVerify
//! allocated up to 64 MiB with `vec!` before they had checked the pointer, and
//! `read_user_bytes` (MkCapsuleLoad's four 16 MiB blobs) with
//! `Vec::with_capacity`. Both abort when the heap cannot give the bytes, and
//! the kernel's allocation failure handler halts the machine. The checks below
//! fail against that code.

use crate::usercopy::buffer::take_buffer;
use crate::usercopy::error::UsercopyError;

const LOCAL_IMAGE: &str = include_str!("../../../src/syscall/microkernel/local_image.rs");
const BYTES: &str = include_str!("../../../src/usercopy/bytes.rs");

/// Byte offset of `needle` in `text`, which must hold it.
fn at(text: &str, needle: &str) -> usize {
    text.find(needle).unwrap_or_else(|| panic!("missing `{needle}`"))
}

#[test]
fn a_buffer_the_heap_can_give_is_empty_with_room_for_all_of_it() {
    for len in [0, 1, 4096, 16 << 20] {
        let buf = take_buffer(len).expect("a heap this size gives it");
        assert!(buf.is_empty());
        assert!(buf.capacity() >= len, "{len} bytes asked, {} given", buf.capacity());
    }
}

#[test]
fn a_buffer_no_heap_can_give_is_refused_not_aborted() {
    for len in [usize::MAX, isize::MAX as usize + 1, usize::MAX / 2 + 4096] {
        assert_eq!(take_buffer(len).err(), Some(UsercopyError::SizeTooLarge), "{len} bytes");
    }
}

#[test]
fn the_local_image_copy_checks_the_range_then_allocates_fallibly() {
    let body = &LOCAL_IMAGE[at(LOCAL_IMAGE, "pub(super) fn copy_in")..];
    let body = &body[..at(body, "\n}\n")];
    let checked = at(body, "crate::usercopy::validate_user_read(ptr, len)");
    let taken = at(body, "crate::usercopy::take_buffer(len)");
    assert!(checked < taken, "the buffer is taken before the range is checked");
    assert!(!body.contains("vec!["), "an infallible allocation is left in copy_in");
}

#[test]
fn read_user_bytes_takes_its_buffer_fallibly() {
    let body = &BYTES[at(BYTES, "pub fn read_user_bytes")..];
    let body = &body[..at(body, "\n}\n")];
    assert!(body.contains("let mut buf = take_buffer(len)?;"));
    assert!(!body.contains("with_capacity"), "an infallible allocation is left");
}
