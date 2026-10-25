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

//! HDMI 2.1's override of the base block's extension count.

use crate::build::{base, seal};
use crate::build_ext::{cta, data, with};
use crate::edid::Edid;

fn eeodb_edid(count: u8, blocks_after: usize) -> Vec<u8> {
    let first = cta(&data(7, &[0x78, count]), &[]);
    let mut ext = vec![first];
    for _ in 0..blocks_after {
        ext.push(cta(&data(2, &[16]), &[]));
    }
    let mut raw = with(base(), &ext);
    // The sink says 1 in byte 126 so a reader without EEODB stops there.
    raw[126] = 1;
    seal(&mut raw[..128]);
    raw
}

#[test]
fn the_eeodb_count_replaces_byte_126() {
    let raw = eeodb_edid(3, 2);
    let e = Edid::parse(&raw).unwrap();
    assert_eq!(e.extension_count(), 3);
    assert_eq!(e.extensions().count(), 3);
}

#[test]
fn a_count_past_the_blocks_handed_over_reads_only_those() {
    let raw = eeodb_edid(255, 1);
    assert_eq!(Edid::parse(&raw).unwrap().extension_count(), 2);
}

#[test]
fn an_eeodb_not_first_in_its_block_is_not_one() {
    let mut blocks = data(2, &[16]);
    blocks.extend(data(7, &[0x78, 3]));
    let raw = with(base(), &[cta(&blocks, &[]), cta(&[], &[]), cta(&[], &[])]);
    let mut raw = raw;
    raw[126] = 1;
    seal(&mut raw[..128]);
    assert_eq!(Edid::parse(&raw).unwrap().extension_count(), 1);
}
