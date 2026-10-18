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

//! OP_SET_SCANOUT. The adapter has one scanout, and it shows the mode from
//! the start of video memory, so the one rectangle it can show the surface
//! at is all of it. A reply of 0 is read off the adapter: it still holds
//! the mode, laid out as every flush writes it.

use super::held::held;
use super::rect::rect_at;
use super::wire::{le_u32, E_DEVICE, E_INVAL, SET_SCANOUT_LEN};
use crate::dispi::{confirm, Rect};
use crate::setup::Driver;

pub fn handle(d: &Driver, sender: u32, body: &[u8]) -> i32 {
    if body.len() != SET_SCANOUT_LEN {
        return E_INVAL;
    }
    let (Some(scanout), Some(resource_id), Some(r)) =
        (le_u32(body, 0), le_u32(body, 4), rect_at(body, 8))
    else {
        return E_INVAL;
    };
    if scanout != 0 || r.width == 0 || r.height == 0 {
        return E_INVAL;
    }
    if let Err(e) = held(d, sender, resource_id) {
        return e;
    }
    if r != (Rect { x: 0, y: 0, width: d.width as u32, height: d.height as u32 }) {
        return E_INVAL;
    }
    match confirm(d.regs, d.width, d.height) {
        Ok(()) => 0,
        Err(_) => E_DEVICE,
    }
}
