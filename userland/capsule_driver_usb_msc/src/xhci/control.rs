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

//! Control transfers on endpoint 0: with no data stage, with a short one
//! in, and the configuration descriptor.

use super::call::call;
use super::wire::{DATA_AT, OP_CONTROL_TRANSFER, OP_GET_CONFIG_DESCRIPTOR};
use crate::protocol::E_IO;

/// The configuration descriptor, up to `out.len()` (at most 512) bytes.
pub fn config_descriptor(xhci: u32, slot: u8, out: &mut [u8]) -> Result<usize, i32> {
    let want = (out.len().min(512) as u16).to_le_bytes();
    let mut resp = [0u8; DATA_AT + 4 + 512];
    let len = call(xhci, OP_GET_CONFIG_DESCRIPTOR, &[slot, 0, want[0], want[1]], &mut resp)?;
    let actual = u16::from_le_bytes([resp[DATA_AT], resp[DATA_AT + 1]]) as usize;
    if len < 4 + actual || actual > out.len() {
        return Err(E_IO);
    }
    out[..actual].copy_from_slice(&resp[DATA_AT + 4..DATA_AT + 4 + actual]);
    Ok(actual)
}

/// A request with no data stage: SET_CONFIGURATION, CLEAR_FEATURE, or the
/// mass-storage reset.
pub fn control_no_data(
    xhci: u32,
    slot: u8,
    request: (u8, u8),
    value: u16,
    index: u16,
) -> Result<(), i32> {
    let mut body = [0u8; 10];
    body[0] = slot;
    body[2] = request.0;
    body[3] = request.1;
    body[4..6].copy_from_slice(&value.to_le_bytes());
    body[6..8].copy_from_slice(&index.to_le_bytes());
    let mut resp = [0u8; DATA_AT + 2];
    call(xhci, OP_CONTROL_TRANSFER, &body, &mut resp).map(|_| ())
}

/// A request with a data stage in of `out.len()` bytes, at most 512: GET MAX
/// LUN. Returns how many bytes came.
pub fn control_in(
    xhci: u32,
    slot: u8,
    request: (u8, u8),
    value: u16,
    index: u16,
    out: &mut [u8],
) -> Result<usize, i32> {
    let want = out.len().min(512) as u16;
    let mut body = [0u8; 10];
    body[0] = slot;
    body[2] = request.0;
    body[3] = request.1;
    body[4..6].copy_from_slice(&value.to_le_bytes());
    body[6..8].copy_from_slice(&index.to_le_bytes());
    body[8..10].copy_from_slice(&want.to_le_bytes());
    let mut resp = [0u8; DATA_AT + 2 + 512];
    let len = call(xhci, OP_CONTROL_TRANSFER, &body, &mut resp)?;
    let actual = u16::from_le_bytes([resp[DATA_AT], resp[DATA_AT + 1]]) as usize;
    if len < 2 + actual || actual > out.len() {
        return Err(E_IO);
    }
    out[..actual].copy_from_slice(&resp[DATA_AT + 2..DATA_AT + 2 + actual]);
    Ok(actual)
}
