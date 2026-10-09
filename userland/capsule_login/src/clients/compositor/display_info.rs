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

//! The canvas the compositor gives its clients. On a HiDPI panel it is half
//! the screen each way, so the overlay must be sized from it, not from the
//! kernel's framebuffer, or its centred lock card lands off the canvas.

use alloc::vec;
use alloc::vec::Vec;

use nonos_libc::{mk_ipc_call, SURFACE_FORMAT_ARGB8888};

use super::constants::{HDR_LEN, MAGIC, VERSION};

const OP_DISPLAY_INFO: u16 = 0x0008;
/// Status, then width, height, stride and format, four bytes each.
const RESP_LEN: usize = 4 + 16;

pub fn display_info(port: u32, request_id: u32) -> Result<(u32, u32), &'static str> {
    let mut tx = Vec::with_capacity(HDR_LEN);
    tx.extend_from_slice(&MAGIC.to_le_bytes());
    tx.extend_from_slice(&VERSION.to_le_bytes());
    tx.extend_from_slice(&OP_DISPLAY_INFO.to_le_bytes());
    tx.extend_from_slice(&0u16.to_le_bytes());
    tx.extend_from_slice(&0u16.to_le_bytes());
    tx.extend_from_slice(&request_id.to_le_bytes());
    tx.extend_from_slice(&0u32.to_le_bytes());

    let mut rx = vec![0u8; HDR_LEN + RESP_LEN];
    let rc = mk_ipc_call(port as u64, tx.as_ptr(), tx.len(), rx.as_mut_ptr(), rx.len());
    if rc < (HDR_LEN + RESP_LEN) as i64 {
        return Err("compositor display_info short reply");
    }
    let word = |at: usize| u32::from_le_bytes([rx[at], rx[at + 1], rx[at + 2], rx[at + 3]]);
    let (status, width, height) = (word(HDR_LEN), word(HDR_LEN + 4), word(HDR_LEN + 8));
    if status != 0 || width == 0 || height == 0 || word(HDR_LEN + 16) != SURFACE_FORMAT_ARGB8888 {
        return Err("compositor display_info refused");
    }
    Ok((width, height))
}
