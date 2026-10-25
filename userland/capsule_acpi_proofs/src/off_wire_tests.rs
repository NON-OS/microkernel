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

//! The shell's power-off request: exactly its 20-byte header is
//! well formed; any change to magic, version, op, flags, reserved,
//! payload length or length is not; any request id is; and a refusal
//! echoes the header with the status at offset 20.

use super::off_wire::*;

fn header(id: u32) -> [u8; HEADER] {
    let mut h = [0u8; HEADER];
    h[..4].copy_from_slice(b"POFF");
    h[4..6].copy_from_slice(&VERSION.to_le_bytes());
    h[6..8].copy_from_slice(&OP_OFF.to_le_bytes());
    h[12..16].copy_from_slice(&id.to_le_bytes());
    h
}

#[test]
fn only_the_exact_header_is_well_formed() {
    assert_eq!(MAGIC, 0x4646_4F50);
    for id in [0, 1, 0xDEAD_BEEF, u32::MAX] {
        assert!(well_formed(&header(id)), "request id {id:#x}");
    }
    let good = header(7);
    for at in (0..HEADER).filter(|i| !(12..16).contains(i)) {
        for bit in 0..8 {
            let mut bad = good;
            bad[at] ^= 1 << bit;
            assert!(!well_formed(&bad), "byte {at} bit {bit}");
        }
    }
    for len in 0..40 {
        let mut long = [0u8; 40];
        long[..HEADER].copy_from_slice(&good);
        assert_eq!(well_formed(&long[..len]), len == HEADER, "length {len}");
        assert_eq!(is_poff(&long[..len]), len >= 4, "magic seen at length {len}");
    }
}

#[test]
fn a_refusal_echoes_the_header_and_its_status() {
    let h = header(42);
    let out = refusal(&h, -1);
    assert_eq!(out[..HEADER], h);
    assert_eq!(i32::from_le_bytes([out[20], out[21], out[22], out[23]]), -1);
    let short = refusal(&h[..6], -22);
    assert_eq!(short[..6], h[..6]);
    assert_eq!(short[6..HEADER], [0; 14], "a short request is padded with zeros");
    assert_eq!(i32::from_le_bytes([short[20], short[21], short[22], short[23]]), -22);
}
