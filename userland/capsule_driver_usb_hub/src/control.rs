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
//! What the hub class needs of a USB host controller driver and of the
//! A control transfer on a slot's EP0 over the host wire: the setup and
//! any OUT data in the request, an IN request's bytes after the reply's
//! length.

use nonos_usb_hub::Setup;

use crate::link::Wire;
use crate::wire::{E_IO, OP_CONTROL_TRANSFER};

pub fn control(w: &Wire, slot: u8, s: Setup, data: &mut [u8]) -> Result<usize, i32> {
    let ([v0, v1], [i0, i1], [l0, l1]) =
        (s.value.to_le_bytes(), s.index.to_le_bytes(), s.length.to_le_bytes());
    let mut body = [0u8; 10 + 512];
    body[..10].copy_from_slice(&[slot, 0, s.request_type, s.request, v0, v1, i0, i1, l0, l1]);
    let out = if s.request_type & 0x80 == 0 { (s.length as usize).min(data.len()) } else { 0 };
    let body = body.get_mut(..10 + out).ok_or(E_IO)?;
    body[10..].copy_from_slice(&data[..out]);
    let mut back = [0u8; 2 + 512];
    let n = w.op(OP_CONTROL_TRANSFER, body, &mut back)?;
    let moved = (u16::from_le_bytes([back[0], back[1]]) as usize).min(n.saturating_sub(2));
    let moved = moved.min(data.len());
    data[..moved].copy_from_slice(&back[2..2 + moved]);
    Ok(moved)
}
