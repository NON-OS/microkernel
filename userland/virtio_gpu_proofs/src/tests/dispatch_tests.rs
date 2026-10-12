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

//! The server's answers to what its client can send, through its real
//! dispatch. The two resource ops no client ever sent (0x0006 create, 0x0007
//! attach backing) are answered as unknown and reach no device: a resource
//! could never be destroyed, and once the table was full a create left the
//! host resource it had just made. A flush is held to the resource's owner
//! and to its bounds.

use super::client::{call, flush};
use super::harness::{driver, harness};
use super::part::{answering, Answer};
use crate::constants::{VG_CMD_RESOURCE_FLUSH, VG_FORMAT_B8G8R8A8_UNORM};
use crate::protocol::{E_BAD_OP, E_BUSY, E_INVAL, OP_FLUSH};
use crate::state::Resource;

#[test]
fn the_retired_resource_ops_are_unknown_and_reach_no_device() {
    let h = harness();
    let part = answering(&h.region, Answer::Spec);
    let (d, _region, _regs) = driver(h, None);
    for op in [0x0006u16, 0x0007] {
        assert_eq!(call(&d, 7, op, &[]), E_BAD_OP, "op {op:#x}");
        for len in [16, 24, 256] {
            assert_eq!(call(&d, 7, op, &vec![0xA5; len]), E_INVAL, "op {op:#x}, {len} bytes");
        }
    }
    assert!(part.seen().is_empty(), "a command reached the device");
}

#[test]
fn a_flush_is_held_to_its_owner_and_its_bounds() {
    let h = harness();
    let part = answering(&h.region, Answer::Spec);
    let (d, _region, _regs) = driver(h, None);
    let format = VG_FORMAT_B8G8R8A8_UNORM;
    let r = Resource { resource_id: 3, owner_pid: 5, width: 64, height: 32, format, in_use: true };
    d.resources.insert(r).expect("room");
    assert_eq!(call(&d, 6, OP_FLUSH, &flush(3, 0, 0, 64, 32)), E_BUSY, "another client's");
    assert_eq!(call(&d, 5, OP_FLUSH, &flush(4, 0, 0, 1, 1)), E_INVAL, "no such resource");
    for (x, y, w, hh) in
        [(0, 0, 65, 32), (1, 0, 64, 1), (0, 32, 1, 1), (0, 0, 0, 1), (u32::MAX, 0, 2, 1)]
    {
        assert_eq!(call(&d, 5, OP_FLUSH, &flush(3, x, y, w, hh)), E_INVAL, "{x},{y} {w}x{hh}");
    }
    assert_eq!(call(&d, 5, OP_FLUSH, &flush(3, 0, 0, 64, 32)[..19]), E_INVAL, "a short body");
    assert!(part.seen().is_empty(), "no refused flush reached the device");
    assert_eq!(call(&d, 5, OP_FLUSH, &flush(3, 0, 0, 64, 32)), 0);
    let seen = part.seen();
    assert_eq!(seen.len(), 1);
    assert_eq!(seen[0][0..4], VG_CMD_RESOURCE_FLUSH.to_le_bytes());
}
