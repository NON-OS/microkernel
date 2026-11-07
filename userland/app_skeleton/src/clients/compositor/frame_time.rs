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

//! OP_FRAME_TIME: when the screen last changed, for a client's presentation
//! feedback (wp_presentation).

use crate::wire::{call_payload, read_u32, HDR_LEN, NCMP_MAGIC};

const OP: u16 = 0x001B;
const BODY_LEN: usize = 24;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FrameTime {
    /// Frames shown so far; it moves on with each frame that drew.
    pub seq: u64,
    /// The kernel's monotonic uptime when the last was shown, in
    /// nanoseconds of millisecond resolution.
    pub at_ns: u64,
    /// The output's refresh period, 0 when it is not known.
    pub refresh_ns: u64,
}

pub fn frame_time(port: u32, request_id: u32) -> Result<FrameTime, &'static str> {
    let mut rx = [0u8; HDR_LEN + 4 + BODY_LEN];
    let total = call_payload(port, NCMP_MAGIC, OP, request_id, &[], &mut rx)?;
    if total < HDR_LEN + 4 + BODY_LEN {
        return Err("frame_time truncated");
    }
    let base = HDR_LEN + 4;
    let wide = |at: usize| -> Result<u64, &'static str> {
        Ok(u64::from(read_u32(&rx, at)?) | u64::from(read_u32(&rx, at + 4)?) << 32)
    };
    Ok(FrameTime { seq: wide(base)?, at_ns: wide(base + 8)?, refresh_ns: wide(base + 16)? })
}
