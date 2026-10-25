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

//! Whatever bytes arrive, the parser answers without panicking or reading
//! past them: every length, every count, every offset, and noise.

use crate::build::{base, dtd_1080p, put, seal};
use crate::build_ext::{cta, data, displayid, with};
use crate::edid::Edid;

/// Touch every answer the crate gives.
pub(super) fn exercise(raw: &[u8]) {
    let Ok(e) = Edid::parse(raw) else { return };
    assert!(e.raw().len() <= raw.len() && e.raw().len() % 128 == 0);
    let mut n = 0usize;
    e.for_each_timing(&mut |_| n += 1);
    e.for_each_audio(&mut |_| n += 1);
    let _ = (e.preferred(), e.native(), e.name(), e.range_limits(), e.tile());
    let _ = (e.max_tmds_khz(), e.physical_mm(), e.basic_audio(), n);
}

pub(super) fn three_blocks() -> Vec<u8> {
    let mut b = base();
    put(&mut b, 0, &dtd_1080p());
    let c = cta(&[data(2, &[16, 4]), data(3, &[3, 12, 0, 0, 0, 0, 60])].concat(), &[dtd_1080p()]);
    with(b, &[c, displayid(&[0x22, 0, 0])])
}

#[test]
fn every_truncation_of_a_three_block_edid() {
    let raw = three_blocks();
    for n in 0..=raw.len() {
        exercise(&raw[..n]);
    }
}

#[test]
fn every_declared_count_against_what_is_there() {
    let raw = three_blocks();
    for count in 0..=255u8 {
        let mut r = raw.clone();
        r[126] = count;
        seal(&mut r[..128]);
        exercise(&r);
        assert_eq!(Edid::parse(&r).unwrap().extension_count(), (count as usize).min(2));
    }
}

#[test]
fn every_dtd_offset_and_every_data_block_header() {
    for d in 0..=255u8 {
        for h in [0x00u8, 0x1F, 0x5F, 0xFF, 0xE0, 0xE1] {
            let mut c = cta(&[], &[]);
            c[2] = d;
            c[4] = h;
            c[5] = 0x78;
            seal(&mut c);
            exercise(&with(base(), &[c]));
        }
    }
}
