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

//! The rail rectangle and reply builders shared by the cases.

use crate::layout::Rect;
use crate::paint::rail_band::Band;
use crate::paint::rail_left_geom::nav_sections;
use crate::rail::net_decode::{HDR_LEN, REPLY_LEN};

pub const RAIL: Rect = Rect { x: 0, y: 40, w: 240, h: 900 };

pub fn list(offset: u32) -> Band {
    nav_sections(RAIL, offset, 4, 3).s_list
}

pub fn lease(state: u8, ip: [u8; 4], prefix: u8) -> [u8; REPLY_LEN] {
    let mut rx = [0u8; REPLY_LEN];
    rx[HDR_LEN] = state;
    rx[HDR_LEN + 1..HDR_LEN + 5].copy_from_slice(&ip);
    rx[HDR_LEN + 5] = prefix;
    rx[HDR_LEN + 6..HDR_LEN + 10].copy_from_slice(&[10, 0, 2, 2]);
    rx
}
