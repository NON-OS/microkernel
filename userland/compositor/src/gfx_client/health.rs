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
//! HEALTHCHECK on the legacy ops (G-73): the poll that tells virtio-gpu
//! this compositor is alive, and whose answer says LOST once after the
//! driver blanked the screen for this compositor's silence.

use alloc::vec;

use super::wire::{call, payload_slice, read_status};

const OP: u16 = 0x0001;
/// The screen was blanked since the last poll: set the scanout again and
/// repaint whole.
pub const LOST: u32 = 1;

/// The answer's flags; a driver from before G-73 answers none.
pub fn health(gfx_port: u32, request_id: u32) -> Result<u32, &'static str> {
    let mut rx = vec![0u8; super::wire::NVGP_HDR_LEN + 4 + 4];
    let n = call(gfx_port, OP, request_id, &[], &mut rx)?;
    let rx = &rx[..n.min(rx.len())];
    if read_status(rx).ok_or("gfx health: short response")? != 0 {
        return Err("gfx health: driver rejected");
    }
    let flags = payload_slice(rx).get(..4).and_then(|b| b.try_into().ok());
    Ok(flags.map_or(0, u32::from_le_bytes))
}
