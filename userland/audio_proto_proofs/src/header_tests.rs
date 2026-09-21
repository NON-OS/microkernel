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

//! The header the service reads by fixed offset.

use nonos_audio_proto::{write_header, HDR_LEN, MAGIC, VERSION};

#[test]
fn the_fields_land_where_the_decoder_looks() {
    let mut out = [0xAAu8; HDR_LEN];
    write_header(&mut out, 7, 0x1234_5678, 12);
    assert_eq!(u32::from_le_bytes([out[0], out[1], out[2], out[3]]), MAGIC);
    assert_eq!(u16::from_le_bytes([out[4], out[5]]), VERSION);
    assert_eq!(u16::from_le_bytes([out[6], out[7]]), 7);
    assert_eq!(u32::from_le_bytes([out[12], out[13], out[14], out[15]]), 0x1234_5678);
    assert_eq!(u32::from_le_bytes([out[16], out[17], out[18], out[19]]), 12);
}

#[test]
fn the_reserved_word_is_cleared() {
    let mut out = [0xFFu8; HDR_LEN];
    write_header(&mut out, 1, 1, 0);
    assert_eq!(u32::from_le_bytes([out[8], out[9], out[10], out[11]]), 0);
}

#[test]
fn a_short_buffer_is_not_half_written() {
    for len in 0..HDR_LEN {
        let mut out = vec![0u8; len];
        write_header(&mut out, 1, 1, 12);
        assert!(out.iter().all(|&b| b == 0), "{len} bytes was partly written");
    }
}

/*
 * The constant spells NAUD as a big endian word, so it lands on the wire as D U
 * A N. Pinned both ways round because the HDA controller's own magic is NHDA,
 * one letter apart, and the two services are reached over separate ports with
 * messages that are otherwise shaped alike.
 */
#[test]
fn the_magic_is_naud_and_not_the_hda_driver_s() {
    assert_eq!(MAGIC.to_be_bytes(), *b"NAUD");
    assert_eq!(MAGIC.to_le_bytes(), *b"DUAN", "which is what a reader of the bytes sees");
    assert_ne!(MAGIC, 0x4e48_4441, "NHDA belongs to the controller, not the mixer");
}
