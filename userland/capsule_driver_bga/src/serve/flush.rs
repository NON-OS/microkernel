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

//! OP_FLUSH: the rectangle copied from the surface into the framebuffer at
//! the same place, the step that changes what the adapter shows.

use super::held::held;
use super::rect::rect_at;
use super::wire::{le_u32, E_INVAL, FLUSH_LEN};
use crate::dispi::copy;
use crate::setup::Driver;

pub fn handle(d: &Driver, sender: u32, body: &[u8]) -> i32 {
    if body.len() != FLUSH_LEN {
        return E_INVAL;
    }
    let (Some(resource_id), Some(r)) = (le_u32(body, 0), rect_at(body, 4)) else {
        return E_INVAL;
    };
    if let Err(e) = held(d, sender, resource_id) {
        return e;
    }
    if copy(&d.surface.pixels, &d.fb, d.width, r) {
        0
    } else {
        E_INVAL
    }
}
