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

//! One tone, asked for in full.

use super::header::{write_header, HDR_LEN};
use super::ops::OP_PLAY_TONE;

/// Frequency, duration and gain, in that order.
pub const TONE_PAYLOAD_LEN: usize = 12;

/// A whole tone request, header included.
pub const TONE_MSG_LEN: usize = HDR_LEN + TONE_PAYLOAD_LEN;

/// Bytes written, or zero if `out` is too short to hold a whole request.
pub fn tone_request(out: &mut [u8], request_id: u32, hz: u32, ms: u32, gain: u16) -> usize {
    if out.len() < TONE_MSG_LEN {
        return 0;
    }
    write_header(out, OP_PLAY_TONE, request_id, TONE_PAYLOAD_LEN as u32);
    out[20..24].copy_from_slice(&hz.to_le_bytes());
    out[24..28].copy_from_slice(&ms.to_le_bytes());
    out[28..30].copy_from_slice(&gain.to_le_bytes());
    out[30..32].copy_from_slice(&0u16.to_le_bytes());
    TONE_MSG_LEN
}
