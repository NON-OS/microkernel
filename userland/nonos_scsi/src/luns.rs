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

//! REPORT LUNS (SPC-5, 6.33) and the LUN formats (SAM-5, 4.7) a host
//! adapter of this layer can name in 16 bits.

/// The header and room for 63 LUNs: far more than any virtual target has.
pub const ALLOC: u32 = 8 + 63 * 8;

/// Each LUN the list names that fits 16 bits, in list order, into `out`;
/// how many were written.
pub fn parse(b: &[u8], out: &mut [u16]) -> usize {
    let Some(head) = b.get(..4) else { return 0 };
    let listed = u32::from_be_bytes([head[0], head[1], head[2], head[3]]) as usize;
    let body = b.get(8..).unwrap_or(&[]);
    let mut n = 0;
    for e in body.chunks_exact(8).take(listed / 8) {
        if n == out.len() {
            break;
        }
        if let Some(lun) = decode(e) {
            out[n] = lun;
            n += 1;
        }
    }
    n
}

/// Peripheral device addressing at bus 0, or flat space addressing; the
/// second level must be empty.
fn decode(e: &[u8]) -> Option<u16> {
    if e[2..].iter().any(|&b| b != 0) {
        return None;
    }
    match e[0] >> 6 {
        0b00 if e[0] & 0x3F == 0 => Some(e[1] as u16),
        0b01 => Some(u16::from(e[0] & 0x3F) << 8 | u16::from(e[1])),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_list() {
        let mut b = vec![0, 0, 0, 32, 0, 0, 0, 0];
        b.extend_from_slice(&[0, 0, 0, 0, 0, 0, 0, 0]);
        b.extend_from_slice(&[0, 3, 0, 0, 0, 0, 0, 0]);
        b.extend_from_slice(&[0x41, 0x02, 0, 0, 0, 0, 0, 0]);
        b.extend_from_slice(&[0xC1, 0x02, 0, 0, 0, 0, 0, 0]);
        b.extend_from_slice(&[0, 9, 0, 0, 0, 0, 0, 0]);
        let mut out = [0u16; 8];
        let n = parse(&b, &mut out);
        assert_eq!(
            &out[..n],
            &[0, 3, 0x102],
            "extended addressing skipped, the list's length kept"
        );
    }

    #[test]
    fn bounds() {
        let mut b = vec![0, 0, 0, 24, 0, 0, 0, 0];
        for lun in 0..3u8 {
            b.extend_from_slice(&[0, lun, 0, 0, 0, 0, 0, 0]);
        }
        let mut out = [0u16; 2];
        assert_eq!(parse(&b, &mut out), 2);
        assert_eq!(parse(&b[..12], &mut out), 0, "an entry cut short is not one");
        assert_eq!(parse(&[], &mut out), 0);
        let mut b2 = b.clone();
        b2[8] = 0x01;
        assert_eq!(parse(&b2[..16], &mut [0; 4]), 0, "a bus other than 0");
        b2[8] = 0;
        b2[10] = 1;
        assert_eq!(parse(&b2[..16], &mut [0; 4]), 0, "a second level");
    }
}
