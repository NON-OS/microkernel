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
//! Whether this machine can play sound, and if not, why, in words a person
//! reads.
//!
//! `OP_OUTPUT_STATUS` takes no payload. Its reply is the header, the status
//! word, then a `u32` output code and a `u32` of flags: bit 0 speakers, bit 1
//! a headphone jack, bit 2 a line out, bit 8 headphones plugged in. The codes
//! from 0 to 6 are the HD Audio driver's own verdicts, passed through by the
//! audio server unchanged; 1 and 7 are the server's, for a machine with no
//! driver and a driver that does not answer.

use super::header::{write_header, HDR_LEN, STATUS_LEN};

pub const OP_OUTPUT_STATUS: u16 = 9;
/// The request was refused because this machine has no output to play on.
/// `OP_OUTPUT_STATUS` says why.
pub const E_NODEV: i32 = -19;

pub const OUTPUT_READY: u32 = 0;
pub const OUTPUT_NO_DEVICE: u32 = 1;
pub const OUTPUT_NEEDS_SOF: u32 = 2;
pub const OUTPUT_NO_CODEC: u32 = 3;
pub const OUTPUT_HDMI_ONLY: u32 = 4;
pub const OUTPUT_NO_PATH: u32 = 5;
pub const OUTPUT_AMD_ACP: u32 = 6;
pub const OUTPUT_NOT_ANSWERING: u32 = 7;

pub const FLAG_SPEAKER: u32 = 1 << 0;
pub const FLAG_HEADPHONE: u32 = 1 << 1;
pub const FLAG_LINE_OUT: u32 = 1 << 2;
pub const FLAG_PLUGGED: u32 = 1 << 8;

pub const OUTPUT_REPLY_LEN: usize = HDR_LEN + STATUS_LEN + 8;

const MESSAGES: [&str; 8] = [
    "Sound plays through this computer's speakers and headphone jack",
    "No sound hardware was found on this computer",
    "This laptop's audio needs Intel's DSP firmware (SOF), which NONOS does not support",
    "The sound controller answered, but no sound chip (codec) is connected to it",
    "Only HDMI or DisplayPort audio was found; NONOS plays through speakers, headphones and line out only",
    "The sound chip has no speaker, headphone or line output NONOS can drive",
    "This computer's audio runs through AMD's audio coprocessor (ACP), which NONOS does not support",
    "The sound driver is not answering",
];

const SHORT: [&str; 8] = [
    "Working",
    "No sound hardware",
    "Needs Intel SOF firmware",
    "No sound chip found",
    "HDMI audio only",
    "No usable output",
    "Needs AMD ACP driver",
    "Driver not answering",
];

/// The sentence for `code`, for a notice line or a settings note.
pub fn output_message(code: u32) -> &'static str {
    MESSAGES.get(code as usize).copied().unwrap_or("No audio output")
}

/// A few words for `code`, for a value column.
pub fn output_short(code: u32) -> &'static str {
    SHORT.get(code as usize).copied().unwrap_or("Unknown")
}

/// Whether `s` is one of the sentences `output_message` gives, so a caller
/// holding only the string can show it as it is.
pub fn is_output_message(s: &str) -> bool {
    MESSAGES.contains(&s)
}

/// An `OP_OUTPUT_STATUS` request: the header alone.
pub fn output_status_request(out: &mut [u8], request_id: u32) -> usize {
    if out.len() < HDR_LEN {
        return 0;
    }
    write_header(out, OP_OUTPUT_STATUS, request_id, 0);
    HDR_LEN
}

/// The reply, as the server sends it. Zero when `out` is too short.
pub fn output_status_reply(out: &mut [u8], request_id: u32, code: u32, flags: u32) -> usize {
    if out.len() < OUTPUT_REPLY_LEN {
        return 0;
    }
    write_header(out, OP_OUTPUT_STATUS, request_id, (STATUS_LEN + 8) as u32);
    out[HDR_LEN..HDR_LEN + 4].copy_from_slice(&0i32.to_le_bytes());
    out[HDR_LEN + 4..HDR_LEN + 8].copy_from_slice(&code.to_le_bytes());
    out[HDR_LEN + 8..HDR_LEN + 12].copy_from_slice(&flags.to_le_bytes());
    OUTPUT_REPLY_LEN
}

/// The code and flags from a reply, or none when it is short or refused.
pub fn read_output_status(resp: &[u8]) -> Option<(u32, u32)> {
    if resp.len() < OUTPUT_REPLY_LEN {
        return None;
    }
    let word = |o: usize| u32::from_le_bytes([resp[o], resp[o + 1], resp[o + 2], resp[o + 3]]);
    if word(HDR_LEN) as i32 != 0 {
        return None;
    }
    Some((word(HDR_LEN + 4), word(HDR_LEN + 8)))
}
