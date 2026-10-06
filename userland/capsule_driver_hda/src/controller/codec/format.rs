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

//! The stream format word (HDA 1.0a section 3.7.1), shared by the stream
//! descriptor's SDnFMT and the converter's SET_STREAM_FORMAT. The two must
//! hold the same value or the converter decodes the link at the wrong rate.
//!
//! Bit 14 picks the 48 kHz or 44.1 kHz base, 13:11 multiply it (x1 to x4),
//! 10:8 divide it (/1 to /8), 6:4 give the sample size and 3:0 the channel
//! count less one.

const BASE_44K1: u16 = 1 << 14;

/// The word for `rate` Hz, `bits` per sample and `channels`, or none when
/// the specification has no encoding for it.
pub const fn encode(rate: u32, bits: u8, channels: u8) -> Option<u16> {
    let (base, mult, div) = match rate {
        8_000 => (0, 1, 6),
        11_025 => (BASE_44K1, 1, 4),
        16_000 => (0, 1, 3),
        22_050 => (BASE_44K1, 1, 2),
        32_000 => (0, 2, 3),
        44_100 => (BASE_44K1, 1, 1),
        48_000 => (0, 1, 1),
        88_200 => (BASE_44K1, 2, 1),
        96_000 => (0, 2, 1),
        176_400 => (BASE_44K1, 4, 1),
        192_000 => (0, 4, 1),
        _ => return None,
    };
    let size = match bits {
        8 => 0,
        16 => 1,
        20 => 2,
        24 => 3,
        32 => 4,
        _ => return None,
    };
    if channels == 0 || channels > 16 {
        return None;
    }
    Some(base | ((mult - 1) << 11) | ((div - 1) << 8) | (size << 4) | (channels as u16 - 1))
}

/// The one format this driver plays: 48 kHz, 16-bit, stereo.
pub const PLAYBACK: u16 = match encode(48_000, 16, 2) {
    Some(f) => f,
    None => 0,
};
