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

//! A timing from its porches, as the DMT table and CVT both state them:
//! active, front porch, sync, back porch, for each direction.

use nonos_edid::Detailed;

/// The four horizontal or vertical numbers: active, front, sync, back.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Axis {
    pub active: u32,
    pub front: u32,
    pub sync: u32,
    pub back: u32,
    pub positive: bool,
}

impl Axis {
    pub const fn new(active: u32, front: u32, sync: u32, back: u32, positive: bool) -> Self {
        Axis { active, front, sync, back, positive }
    }
}

pub const fn timing(pixel_khz: u32, h: Axis, v: Axis) -> Detailed {
    Detailed {
        pixel_khz,
        h_active: h.active,
        h_blank: h.front + h.sync + h.back,
        h_front: h.front,
        h_sync: h.sync,
        v_active: v.active,
        v_blank: v.front + v.sync + v.back,
        v_front: v.front,
        v_sync: v.sync,
        h_positive: h.positive,
        v_positive: v.positive,
        interlaced: false,
        width_mm: 0,
        height_mm: 0,
    }
}

const NONE: Axis = Axis::new(0, 0, 0, 0, false);

/// The zero timing a table slot holds before it is filled.
pub const EMPTY: Detailed = timing(0, NONE, NONE);
