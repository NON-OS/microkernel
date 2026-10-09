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

//! The streaming PNG path: 16-bit tRNS keys compared at full depth, and
//! damaged streams that must fail cleanly.
use nonos_toolkit::image::types::DecodeError;

use crate::png_build::png;
use crate::png_rows::{decode, pass_rows, H, W};

#[test]
fn a_16_bit_trns_key_matches_all_sixteen_bits() {
    let raw = [0u8, 0x01, 0x02, 0x01, 0x03];
    let file = png((2, 1, 16, 0, 0), &[(b"tRNS", &[0x01, 0x02])], &raw, 64);
    assert_eq!(decode(&file, 2).unwrap(), [0x0001_0101, 0xff01_0101]);
    let rgb = [0u8, 0, 1, 0, 2, 0, 3, 0, 1, 0, 2, 0, 4];
    let file = png((2, 1, 16, 2, 0), &[(b"tRNS", &[0, 1, 0, 2, 0, 3])], &rgb, 64);
    assert_eq!(decode(&file, 2).unwrap(), [0, 0xff00_0000]);
}

#[test]
fn damaged_streams_fail_without_panicking() {
    let raw = pass_rows([0, 0, 1, 1]);
    let good = png((W as u32, H as u32, 8, 0, 0), &[], &raw, 16);
    /* The stream stops before the last scanline is complete. */
    let short = png((W as u32, H as u32, 8, 0, 0), &[], &raw[..raw.len() - 3], 16);
    assert_eq!(decode(&short, W * H), Err(DecodeError::Truncated));
    /* One flipped bit in an IDAT payload breaks its CRC. */
    let mut bad = good.clone();
    bad[60] ^= 0x10;
    assert_eq!(decode(&bad, W * H), Err(DecodeError::BadMagic));
    /* Data past the last scanline is not needed and not an error. */
    let mut long = raw.clone();
    long.extend_from_slice(&[0u8; 40]);
    assert!(decode(&png((W as u32, H as u32, 8, 0, 0), &[], &long, 16), W * H).is_ok());
    assert_eq!(decode(&good, W * H - 1), Err(DecodeError::OutputTooSmall));
}
