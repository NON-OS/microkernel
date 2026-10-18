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

//! OP_FLUSH: SVGA_CMD_UPDATE for the rectangle, so the host redraws it.

use super::rect::rect_at;
use crate::constants::mode::PRIMARY_RESOURCE;
use crate::fifo::update;
use crate::protocol::{le_u32, E_DEVICE, E_INVAL};
use crate::setup::Driver;

const BODY_LEN: usize = 20;

pub fn flush(d: &mut Driver, body: &[u8]) -> i32 {
    if body.len() != BODY_LEN || le_u32(body, 0) != Some(PRIMARY_RESOURCE) {
        return E_INVAL;
    }
    let Some(r) = rect_at(d, body, 4) else { return E_INVAL };
    match update(&d.regs, &d.fifo, r.x, r.y, r.w, r.h) {
        Ok(()) => 0,
        Err(_) => E_DEVICE,
    }
}
