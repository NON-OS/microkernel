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

//! The bounds-checked field readers behind both parsers, and the medium
//! rule: every op but the health check answers only the kernel's client
//! or a sender the kernel says holds StoreWrite.

use crate::protocol::{read_u32_le, read_u64_le};

#[test]
fn field_readers_are_inbounds_or_none_never_panicking() {
    let buf: Vec<u8> = (0u8..16).collect();
    assert_eq!(read_u32_le(&buf, 0), Some(u32::from_le_bytes([0, 1, 2, 3])));
    assert_eq!(read_u64_le(&buf, 8), Some(u64::from_le_bytes([8, 9, 10, 11, 12, 13, 14, 15])));
    assert_eq!(read_u32_le(&buf, 13), None);
    assert_eq!(read_u64_le(&buf, 9), None);
    assert_eq!(read_u32_le(&buf, usize::MAX), None, "offset overflow must be None");
    assert_eq!(read_u64_le(&buf, usize::MAX - 3), None);
    assert_eq!(read_u32_le(&[], 0), None);
}

#[test]
fn the_medium_answers_only_the_kernel_or_a_sender_holding_store_write() {
    use crate::protocol::{OP_CAPACITY, OP_FLUSH, OP_HEALTHCHECK, OP_READ_BLOCKS, OP_WRITE_BLOCKS};
    use crate::server::acl::rule::allows;
    for op in [OP_WRITE_BLOCKS, OP_FLUSH, OP_READ_BLOCKS, OP_CAPACITY, 0xffff] {
        assert!(allows(op, 0, false), "the kernel client must never be refused");
        assert!(!allows(op, 7, false), "a sender without StoreWrite reached the medium");
        assert!(allows(op, 7, true), "a sender the kernel vouches for was refused");
    }
    assert!(allows(OP_HEALTHCHECK, 7, false), "a health check must stay open");
}
