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
//! The `OP_OUTPUT_STATUS` reply body, after the status word:
//!
//! ```text
//! u32 verdict, u16 codec_vendor, u16 codec_device, u8 outputs, u8 plugged, u16 reserved
//! ```
//!
//! `verdict` is `controller::verdict::Verdict` as a number, the same number
//! `nonos_audio_proto::output` names for applications. `outputs` has bit 0
//! for speakers, bit 1 for a headphone jack and bit 2 for a line out;
//! `plugged` is 1 while headphones are in.

use super::limits::OUTPUT_STATUS_PAYLOAD_LEN;

pub const OUT_SPEAKER: u8 = 1 << 0;
pub const OUT_HEADPHONE: u8 = 1 << 1;
pub const OUT_LINE: u8 = 1 << 2;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct OutputStatus {
    pub verdict: u32,
    pub codec_vendor: u16,
    pub codec_device: u16,
    pub outputs: u8,
    pub plugged: bool,
}

impl OutputStatus {
    pub const fn silent(verdict: u32) -> Self {
        OutputStatus { verdict, codec_vendor: 0, codec_device: 0, outputs: 0, plugged: false }
    }
}

/// Lay `s` into the first `OUTPUT_STATUS_PAYLOAD_LEN` bytes of `out`.
pub fn write_output_status(out: &mut [u8], s: &OutputStatus) {
    if out.len() < OUTPUT_STATUS_PAYLOAD_LEN {
        return;
    }
    out[0..4].copy_from_slice(&s.verdict.to_le_bytes());
    out[4..6].copy_from_slice(&s.codec_vendor.to_le_bytes());
    out[6..8].copy_from_slice(&s.codec_device.to_le_bytes());
    out[8] = s.outputs;
    out[9] = s.plugged as u8;
    out[10..12].copy_from_slice(&0u16.to_le_bytes());
}
