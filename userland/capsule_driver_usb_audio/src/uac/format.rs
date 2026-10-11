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

//! Whether an AudioStreaming alternate setting carries what the audio
//! server sends: PCM, two channels of 16 bits in two-byte subframes, at
//! 48 kHz (USB Audio 1.0 Format Type I, 2.2 and 2.3.1,
//! as recalled, unverified).

pub const RATE_HZ: u32 = 48_000;
/// bytes per frame: two channels of two bytes.
pub const FRAME_BYTES: u32 = 4;

const FORMAT_TYPE_I: u8 = 1;
const PCM: u16 = 0x0001;

fn rate_at(d: &[u8], at: usize) -> Option<u32> {
    let b = d.get(at..at + 3)?;
    Some(b[0] as u32 | (b[1] as u32) << 8 | (b[2] as u32) << 16)
}

/// A class-specific AS_GENERAL descriptor (subtype 1): its wFormatTag.
pub fn is_pcm(as_general: &[u8]) -> bool {
    as_general.len() >= 7 && u16::from_le_bytes([as_general[5], as_general[6]]) == PCM
}

/// A FORMAT_TYPE descriptor (subtype 2) for stereo 16-bit PCM that
/// plays 48 kHz: a listed rate, or a continuous range holding it.
pub fn plays_48k_stereo(ft: &[u8]) -> bool {
    if ft.len() < 8 || ft[3] != FORMAT_TYPE_I || ft[4] != 2 || ft[5] != 2 || ft[6] != 16 {
        return false;
    }
    match ft[7] {
        0 => matches!(
            (rate_at(ft, 8), rate_at(ft, 11)),
            (Some(lo), Some(hi)) if lo <= RATE_HZ && RATE_HZ <= hi
        ),
        n => (0..n as usize).any(|i| rate_at(ft, 8 + 3 * i) == Some(RATE_HZ)),
    }
}
