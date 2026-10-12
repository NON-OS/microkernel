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

//! DisplayID 2.0 beyond type VII: type VIII DMT codes (one byte only),
//! type IX formula timings past the base block's 4095 limit, native size in
//! whole millimetres with the counts stored less one, and a refresh range
//! past 255 Hz.

use crate::build::base;
use crate::build_did::{block, displayid};
use crate::build_ext::with;
use crate::edid::Edid;

#[test]
fn type_viii_and_ix_name_dmt_modes_and_large_formula_timings() {
    let mut ix = [0u8; 6];
    ix[0] = 0x81; // preferred, CVT reduced blanking
    ix[1..3].copy_from_slice(&(7680u16 - 1).to_le_bytes());
    ix[3..5].copy_from_slice(&(4320u16 - 1).to_le_bytes());
    ix[5] = 59;
    let mut blocks = block(0x23, 0, &[0x52]);
    blocks.extend(block(0x23, 0x08, &[0x52, 0x00])); // two-byte codes: skipped
    blocks.extend(block(0x24, 0, &ix));
    let raw = with(base(), &[displayid(&blocks)]);
    let mut v = Vec::new();
    Edid::parse(&raw).unwrap().for_each_timing(&mut |t| v.push((t.width, t.height, t.preferred)));
    assert_eq!(v, [(7680, 4320, true), (1920, 1080, false)], "formula timings walk before DMT IDs");
}

#[test]
fn native_size_and_a_refresh_range_past_255_hz() {
    let mut params = [0u8; 29];
    params[..2].copy_from_slice(&600u16.to_le_bytes()); // mm, revision bit 7
    params[2..4].copy_from_slice(&340u16.to_le_bytes());
    params[4..6].copy_from_slice(&(3840u16 - 1).to_le_bytes());
    params[6..8].copy_from_slice(&(2160u16 - 1).to_le_bytes());
    let mut range = [0u8; 9];
    (range[6], range[7], range[8]) = (48, 360u32 as u8, 1); // 360 = 0x168
    let mut blocks = block(0x21, 0x80, &params);
    blocks.extend(block(0x25, 0, &range));
    let raw = with(base(), &[displayid(&blocks)]);
    let e = Edid::parse(&raw).unwrap();
    let n = e.displayid_native().unwrap();
    assert_eq!((n.width, n.height, n.width_mm, n.height_mm), (3840, 2160, 600, 340));
    assert_eq!(e.displayid_refresh(), Some((48, 360)));
}
