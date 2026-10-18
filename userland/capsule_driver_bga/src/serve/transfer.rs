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

//! OP_TRANSFER_TO_HOST. Virtio-gpu copies the backing into the device's
//! own copy of the resource. Here the surface is the resource, so there is
//! no second copy to fill: flush reads the surface itself. What a transfer
//! is held to is what flush will read: the rectangle within the mode, and
//! `offset` naming where in the surface it starts.

use super::held::held;
use super::rect::rect_at;
use super::wire::{le_u32, le_u64, E_INVAL, TRANSFER_LEN};
use crate::dispi::place;
use crate::setup::Driver;

pub fn handle(d: &Driver, sender: u32, body: &[u8]) -> i32 {
    if body.len() != TRANSFER_LEN {
        return E_INVAL;
    }
    let (Some(resource_id), Some(r), Some(offset)) =
        (le_u32(body, 0), rect_at(body, 4), le_u64(body, 24))
    else {
        return E_INVAL;
    };
    if let Err(e) = held(d, sender, resource_id) {
        return e;
    }
    match place(&d.surface.pixels, d.width, r) {
        Some(at) if at == offset => 0,
        _ => E_INVAL,
    }
}
