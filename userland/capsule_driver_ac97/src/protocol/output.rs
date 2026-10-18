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

//! `OP_OUTPUT_STATUS`'s body, laid out as driver.hda0 lays it:
//!
//! ```text
//! u32 verdict, u16 codec_vendor, u16 codec_device, u8 outputs, u8 plugged, u16 reserved
//! ```
//!
//! An AC'97 controller that came up plays, so the verdict is always ready;
//! one that did not never registers its service. The output is the codec's
//! line out, which is what a virtual machine's AC'97 plays to the host.

use crate::codec::Ident;

pub const OUTPUT_STATUS_LEN: usize = 12;
/// `nonos_audio_proto::output::OUTPUT_READY`.
pub const VERDICT_READY: u32 = 0;
/// `nonos_audio_proto::output::FLAG_LINE_OUT`, as the driver's outputs byte.
pub const OUT_LINE: u8 = 1 << 2;

pub fn write_output_status(out: &mut [u8], id: &Ident) {
    out[0..4].copy_from_slice(&VERDICT_READY.to_le_bytes());
    out[4..6].copy_from_slice(&id.vendor.to_le_bytes());
    out[6..8].copy_from_slice(&id.device.to_le_bytes());
    out[8] = OUT_LINE;
    out[9] = 0;
    out[10..12].copy_from_slice(&0u16.to_le_bytes());
}
