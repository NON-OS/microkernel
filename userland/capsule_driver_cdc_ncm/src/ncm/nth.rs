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

//! The header of a received NTB (USB NCM 1.0, 3.2, as recalled,
//! unverified). NTH16 "NCMH" is dwSignature, wHeaderLength 12, wSequence,
//! wBlockLength, wNdpIndex; NTH32 "ncmh" is dwSignature, wHeaderLength 16,
//! wSequence, dwBlockLength, dwNdpIndex.

use super::format::{Format, NTB16, NTB32};

/// The block in `raw`, as its format, its bytes and its first NDP's index.
/// NTB32 only from a device that offered it. The header must be its
/// format's own length, the block no longer than `rx_max` (the input size
/// the device was given) and all of it in the transfer; a block length of
/// 0 leaves the block's end to the transfer's. Every NDP index, the
/// first among them, must lie past the header: ntb_in.rs's walk holds it.
pub fn block(raw: &[u8], rx_max: usize, ntb32: bool) -> Option<(&'static Format, &[u8], usize)> {
    let f = match le32(raw, 0)? {
        s if s == NTB16.nth_sign as usize => &NTB16,
        s if ntb32 && s == NTB32.nth_sign as usize => &NTB32,
        _ => return None,
    };
    if le16(raw, 4)? != f.nth_len {
        return None;
    }
    let body = match f.field(raw, 8)? {
        0 => raw,
        n => raw.get(..n)?,
    };
    let first = f.field(raw, if f.wide { 12 } else { 10 })?;
    (body.len() <= rx_max).then_some((f, body, first))
}

pub fn le16(b: &[u8], at: usize) -> Option<usize> {
    let w = b.get(at..at.checked_add(2)?)?;
    Some(u16::from_le_bytes([w[0], w[1]]) as usize)
}

pub fn le32(b: &[u8], at: usize) -> Option<usize> {
    let w = b.get(at..at.checked_add(4)?)?;
    Some(u32::from_le_bytes([w[0], w[1], w[2], w[3]]) as usize)
}
