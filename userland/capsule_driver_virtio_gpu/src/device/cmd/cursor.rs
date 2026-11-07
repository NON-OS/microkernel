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

//! struct virtio_gpu_update_cursor, for UPDATE_CURSOR and MOVE_CURSOR: the
//! header, the position (scanout, x, y, padding), the resource (0 hides
//! the cursor), the hot spot and padding. The position is the image's top
//! left corner, as the device and Linux's driver take it; one left of or
//! above the screen is sent as its two's complement, as Linux sends it.

use super::hdr::{Hdr, HDR_LEN};
use crate::constants::{VG_CMD_MOVE_CURSOR, VG_CMD_UPDATE_CURSOR};

pub const REQ_LEN: usize = HDR_LEN + 32;

/// Where the cursor goes and what it shows.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Place {
    pub scanout: u32,
    pub x: i32,
    pub y: i32,
}

/// UPDATE_CURSOR: `resource` (0: none) with its hot spot, at `at`.
pub fn update(at: Place, resource: u32, hot: (u32, u32)) -> [u8; REQ_LEN] {
    encode(VG_CMD_UPDATE_CURSOR, at, resource, hot)
}

/// MOVE_CURSOR: the shape the scanout has, to `at`.
pub fn movement(at: Place) -> [u8; REQ_LEN] {
    encode(VG_CMD_MOVE_CURSOR, at, 0, (0, 0))
}

fn encode(kind: u32, at: Place, resource: u32, (hot_x, hot_y): (u32, u32)) -> [u8; REQ_LEN] {
    let mut req = [0u8; REQ_LEN];
    Hdr::new(kind, 0).write(&mut req[..HDR_LEN]);
    let words = [at.scanout, at.x as u32, at.y as u32, 0, resource, hot_x, hot_y, 0];
    for (i, w) in words.iter().enumerate() {
        let off = HDR_LEN + i * 4;
        req[off..off + 4].copy_from_slice(&w.to_le_bytes());
    }
    req
}
