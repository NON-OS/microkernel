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

//! ASCII armor (RFC 9580 6.2): the base64 between a BEGIN and an END line,
//! headers skipped, and the CRC-24 checked when one is given.

use alloc::vec::Vec;

use super::base64::decode;

/// The binary in the first armored block of `text`.
pub fn dearmor(text: &[u8]) -> Option<Vec<u8>> {
    let mut lines = text.split(|&b| b == b'\n').map(|l| l.strip_suffix(b"\r").unwrap_or(l));
    lines.find(|l| l.starts_with(b"-----BEGIN PGP "))?;
    // Armor headers run to the first blank line.
    for l in lines.by_ref() {
        if l.iter().all(|b| b.is_ascii_whitespace()) {
            break;
        }
    }
    let (mut body, mut crc) = (Vec::new(), None);
    for l in lines {
        if l.starts_with(b"-----END PGP ") {
            let data = decode(&body)?;
            return match crc {
                Some(c) => (crc24(&data) == c).then_some(data),
                None => Some(data),
            };
        }
        match l.strip_prefix(b"=") {
            Some(sum) => {
                crc = Some(
                    decode(sum)
                        .filter(|s| s.len() == 3)
                        .map(|s| u32::from_be_bytes([0, s[0], s[1], s[2]]))?,
                )
            }
            None => body.extend(l.iter().filter(|b| !b.is_ascii_whitespace())),
        }
    }
    None
}

fn crc24(data: &[u8]) -> u32 {
    let mut crc: u32 = 0xB7_04CE;
    for &b in data {
        crc ^= u32::from(b) << 16;
        for _ in 0..8 {
            crc <<= 1;
            if crc & 0x100_0000 != 0 {
                crc ^= 0x186_4CFB;
            }
        }
    }
    crc & 0xFF_FFFF
}
