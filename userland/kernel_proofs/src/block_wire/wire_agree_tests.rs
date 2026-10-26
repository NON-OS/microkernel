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

//! The kernel's client speaks the drivers' block wire (nonos_blk_wire): the
//! same op numbers and the same READ and WRITE head, so a request the kernel
//! sends is the one a driver reads. And its cap on one transfer still lets a
//! disk of 4096-byte LBAs be addressed.

use super::header::{MAX_TRANSFER_BYTES, RW_HEADER_LEN};
use super::header::{OP_FLUSH, OP_GEOMETRY, OP_HEALTHCHECK, OP_IDENTIFY, OP_READ, OP_WRITE};
use super::lba_map::{addressable, SECTOR};

const OPS: &str = include_str!("../../../nonos_blk_wire/src/ops.rs");
const LIMITS: &str = include_str!("../../../nonos_blk_wire/src/limits.rs");

#[test]
fn every_op_is_the_drivers() {
    let ops = [
        ("HEALTHCHECK", OP_HEALTHCHECK),
        ("GEOMETRY", OP_GEOMETRY),
        ("READ", OP_READ),
        ("WRITE", OP_WRITE),
        ("FLUSH", OP_FLUSH),
        ("IDENTIFY", OP_IDENTIFY),
    ];
    for (name, op) in ops {
        assert!(OPS.contains(&format!("pub const OP_{name}: u16 = {op};")), "OP_{name}");
    }
    assert!(LIMITS.contains(&format!("pub const RW_HEADER_LEN: usize = {RW_HEADER_LEN};")));
}

#[test]
fn the_transfer_cap_addresses_every_lba_size_up_to_a_page() {
    let cap = MAX_TRANSFER_BYTES;
    assert!((cap as usize).is_multiple_of(4096));
    for lba in [512, 1024, 2048, 4096] {
        assert!(addressable(lba, cap), "{lba}-byte LBAs");
    }
    assert!(!addressable(SECTOR as u32 / 2, cap) && !addressable(cap * 2, cap));
}
