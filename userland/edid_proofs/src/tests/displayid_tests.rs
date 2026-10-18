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

//! DisplayID 2.0 in an EDID extension: type VII timings and tiles.

use crate::build::base;
use crate::build_did::block;
use crate::build_ext::{displayid, type7, with};
use crate::edid::Edid;
use crate::source::Source;

pub(super) fn panel() -> Vec<u8> {
    let t = type7(533_250, [2880, 160, 48, 32, 1800, 51, 3, 6], 0x80);
    let mut tile = [0u8; 22];
    tile[0] = 0x80;
    tile[1] = 0x10; // two across, one down
    tile[2] = 0x10; // the right hand tile
    tile[4..6].copy_from_slice(&2559u16.to_le_bytes());
    tile[6..8].copy_from_slice(&2879u16.to_le_bytes());
    tile[13..16].copy_from_slice(b"DEL");
    tile[16..18].copy_from_slice(&0x4123u16.to_le_bytes());
    tile[18..22].copy_from_slice(&7u32.to_le_bytes());
    let mut blocks = block(0x22, 0, &t);
    blocks.extend(block(0x28, 0, &tile));
    with(base(), &[displayid(&blocks)])
}

#[test]
fn a_type_vii_timing_reads_back_with_every_field_plus_one() {
    let raw = panel();
    let e = Edid::parse(&raw).unwrap();
    let p = e.preferred().unwrap();
    assert_eq!(
        (p.pixel_khz, p.h_active, p.h_blank, p.h_front, p.h_sync),
        (533_250, 2880, 160, 48, 32)
    );
    assert_eq!((p.v_active, p.v_blank, p.v_front, p.v_sync), (1800, 51, 3, 6));
    assert!(p.h_positive && p.v_positive);
    let mut v = Vec::new();
    e.for_each_timing(&mut |t| v.push(t));
    assert_eq!(v.len(), 1);
    assert!(v[0].preferred && v[0].source == Source::DisplayId);
}

#[test]
fn a_bad_section_checksum_hides_the_section() {
    let mut raw = panel();
    raw[128 + 10] ^= 1;
    raw[255] = raw[255].wrapping_sub(1); // keep the EDID block's sum
    let e = Edid::parse(&raw).unwrap();
    assert_eq!(e.extensions().count(), 1, "the EDID block itself is fine");
    assert!(e.tile().is_none() && e.preferred().is_none());
}

#[test]
fn a_block_running_past_the_payload_ends_the_walk() {
    let t = type7(148_500, [1920, 280, 88, 44, 1080, 45, 4, 5], 0x80);
    let mut b = block(0x22, 0, &t);
    b[2] = 200;
    let raw = with(base(), &[displayid(&b)]);
    assert!(Edid::parse(&raw).unwrap().preferred().is_none());
}
