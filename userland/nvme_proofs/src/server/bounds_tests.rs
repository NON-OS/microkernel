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

//! The request bounds the I/O queue keeps: the namespace's capacity and the
//! sectors one command may move, fixed at bring-up. The server's parser holds
//! every read and write to them, so a request past the end of the disk or
//! past the data buffer is refused before anything reaches the controller.

use crate::fake::up;
use crate::protocol::{E_INVAL, E_NXIO};
use crate::server::handlers::parse_rw_within;

/// A read or write body: `n` sectors at `lba`.
fn body(lba: u64, n: u32) -> [u8; 12] {
    let mut b = [0u8; 12];
    b[..8].copy_from_slice(&lba.to_le_bytes());
    b[8..].copy_from_slice(&n.to_le_bytes());
    b
}

#[test]
fn a_request_is_held_to_the_queues_own_bounds() {
    let up = up();
    let (cap, max) = (up.io.capacity_sectors, up.io.max_sectors());
    assert_eq!((cap, max), (1 << 20, 64), "the geometry bring-up was given");
    let parse = |lba, n| parse_rw_within(&body(lba, n), cap, max);
    assert_eq!(parse(0, max), Ok((0, max)));
    assert_eq!(parse(cap - 1, 1), Ok((cap - 1, 1)), "the last sector");
    assert_eq!(parse(cap - 1, 2), Err(E_NXIO), "one past the end");
    assert_eq!(parse(u64::MAX, 1), Err(E_INVAL), "lba + count wraps");
    for n in [0, max + 1, u32::MAX] {
        assert_eq!(parse(0, n), Err(E_INVAL), "{n} sectors");
    }
}
