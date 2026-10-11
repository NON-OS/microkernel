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

//! The presents, with driver.virtio_gpu0's checks in its order: the body's
//! length, the resource, its holder, then the rect.

use crate::gfx::{copy, Rect};
use crate::protocol::{
    le_u32, le_u64, E_INVAL, FLUSH_REQ_LEN, SET_SCANOUT_REQ_LEN, TRANSFER_TO_HOST_REQ_LEN,
};
use crate::setup::Driver;

/// The fields at `at` as a resource or scanout id and a rect.
fn rect(body: &[u8], at: usize) -> Option<Rect> {
    let f = |i: usize| le_u32(body, at + i * 4);
    Some(Rect { x: f(0)?, y: f(1)?, w: f(2)?, h: f(3)? })
}

pub fn transfer(d: &mut Driver, sender: u32, body: &[u8]) -> Result<(), i32> {
    if body.len() != TRANSFER_TO_HOST_REQ_LEN {
        return Err(E_INVAL);
    }
    let (r, offset) = rect(body, 4).zip(le_u64(body, 24)).ok_or(E_INVAL)?;
    d.screen.held(sender, le_u32(body, 0).ok_or(E_INVAL)?)?;
    d.screen.inside(r)?;
    d.screen.source(r, offset)?;
    // SAFETY: the shadow and the VRAM map each span the mode, and the three
    // checks above are `copy`'s precondition.
    unsafe { copy(d.shadow.va, d.fb_va, d.mode.stride, r, offset) };
    Ok(())
}

pub fn scanout(d: &mut Driver, sender: u32, body: &[u8]) -> Result<(), i32> {
    if body.len() != SET_SCANOUT_REQ_LEN {
        return Err(E_INVAL);
    }
    let r = rect(body, 8).ok_or(E_INVAL)?;
    d.screen.held(sender, le_u32(body, 4).ok_or(E_INVAL)?)?;
    d.screen.whole(le_u32(body, 0).ok_or(E_INVAL)?, r)
}

/// VRAM is what the adapter scans out, so the transfer already showed the
/// rect; a flush has only to be a valid one.
pub fn flush(d: &mut Driver, sender: u32, body: &[u8]) -> Result<(), i32> {
    if body.len() != FLUSH_REQ_LEN {
        return Err(E_INVAL);
    }
    let r = rect(body, 4).ok_or(E_INVAL)?;
    d.screen.held(sender, le_u32(body, 0).ok_or(E_INVAL)?)?;
    d.screen.inside(r)
}
