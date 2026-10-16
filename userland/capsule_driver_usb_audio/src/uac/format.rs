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
//! Whether an AudioStreaming alternate setting carries what the driver
//! moves: PCM, 16 bits in two-byte subframes, at 48 kHz. 1.0 lists its
//! rates in Format Type I (2.2 and 2.3.1); 2.0 moves the channel count to
//! AS_GENERAL and the rate to a clock (as recalled, unverified).

pub const RATE_HZ: u32 = 48_000;
/// Bytes per sample.
pub const SAMPLE_BYTES: u32 = 2;

const FORMAT_TYPE_I: u8 = 1;
const PCM: u16 = 0x0001;
/// 2.0 bmFormats bit 0: PCM.
const PCM_BIT: u32 = 1;

fn rate_at(d: &[u8], at: usize) -> Option<u32> {
    let b = d.get(at..at + 3)?;
    Some(b[0] as u32 | (b[1] as u32) << 8 | (b[2] as u32) << 16)
}

/// A class-specific AS_GENERAL descriptor (subtype 1) of `version`: PCM,
/// and for 2.0 its channel count.
pub fn general(version: u8, d: &[u8]) -> (bool, Option<u8>) {
    match version {
        1 => (d.len() >= 7 && u16::from_le_bytes([d[5], d[6]]) == PCM, None),
        2 if d.len() >= 11 => {
            let formats = u32::from_le_bytes([d[6], d[7], d[8], d[9]]);
            (d[5] == FORMAT_TYPE_I && formats & PCM_BIT != 0, Some(d[10]))
        }
        _ => (false, None),
    }
}

/// A FORMAT_TYPE descriptor (subtype 2) of `version` for 16-bit PCM:
/// `Some` with 1.0's channel count when it plays 48 kHz (a listed rate or
/// a continuous range holding it), `Some(None)` for 2.0, whose rate is
/// asked of its clock at bind.
pub fn format_i(version: u8, ft: &[u8]) -> Option<Option<u8>> {
    match version {
        1 if ft.len() >= 8 && ft[3] == FORMAT_TYPE_I && ft[5] == 2 && ft[6] == 16 => {
            // bSamFreqType rates of three bytes, or a range of two for 0
            // (1.0 Format 2.2.5, as recalled, unverified), all within
            // bLength: a count the descriptor does not hold is a lie.
            let rates = if ft[7] == 0 { 2 } else { ft[7] as usize };
            if ft.len() < 8 + 3 * rates {
                return None;
            }
            let rated = match ft[7] {
                0 => matches!(
                    (rate_at(ft, 8), rate_at(ft, 11)),
                    (Some(lo), Some(hi)) if lo <= RATE_HZ && RATE_HZ <= hi
                ),
                n => (0..n as usize).any(|i| rate_at(ft, 8 + 3 * i) == Some(RATE_HZ)),
            };
            rated.then_some(Some(ft[4]))
        }
        2 if ft.len() >= 6 && ft[3] == FORMAT_TYPE_I && ft[4] == 2 && ft[5] == 16 => Some(None),
        _ => None,
    }
}
