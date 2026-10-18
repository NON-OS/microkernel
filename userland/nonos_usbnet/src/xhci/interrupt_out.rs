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
//! One report to an interrupt OUT endpoint (OP_INTERRUPT_OUT, 0x25), its
//! ring made by ALLOC_TRANSFER_RING. The host waits for the device to take
//! it and answers `moved_le16, 0, 0`.

use super::call::call;
use super::wire::{DATA_AT, E_INVAL, E_IO, OP_INTERRUPT_OUT};

/// The most one report carries: a full-speed interrupt packet.
pub const REPORT_MAX: usize = 64;

pub fn interrupt_out(xhci: u32, slot: u8, dci: u8, data: &[u8]) -> Result<usize, i32> {
    if data.is_empty() || data.len() > REPORT_MAX {
        return Err(E_INVAL);
    }
    let mut body = [0u8; 4 + REPORT_MAX];
    body[..2].copy_from_slice(&[slot, dci]);
    body[2..4].copy_from_slice(&(data.len() as u16).to_le_bytes());
    body[4..4 + data.len()].copy_from_slice(data);
    let mut resp = [0u8; DATA_AT + 4];
    if call(xhci, OP_INTERRUPT_OUT, &body[..4 + data.len()], &mut resp)? < 4 {
        return Err(E_IO);
    }
    Ok(u16::from_le_bytes([resp[DATA_AT], resp[DATA_AT + 1]]) as usize)
}
