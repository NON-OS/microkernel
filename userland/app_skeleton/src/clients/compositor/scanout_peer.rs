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

//! OP_SCANOUT_PEER: the display driver a swapchain client may share its
//! buffers with, read only, so a full-screen opaque one is scanned out as
//! it is. 0 when the driver cannot; a buffer not shared is composed.

use crate::wire::{call_payload, read_u32, HDR_LEN, NCMP_MAGIC};

const OP: u16 = 0x0020;

pub fn scanout_peer(port: u32, request_id: u32) -> Result<u32, &'static str> {
    let mut rx = [0u8; HDR_LEN + 4 + 4];
    let total = call_payload(port, NCMP_MAGIC, OP, request_id, &[], &mut rx)?;
    if total < HDR_LEN + 8 {
        return Err("scanout_peer truncated");
    }
    read_u32(&rx, HDR_LEN + 4)
}
