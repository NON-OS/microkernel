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

//! Standard timings (Table 3.19), the 0xFA descriptor and CVT codes.

use crate::base::{cvt3, standard};
use crate::build::{base, display, put, seal};
use crate::edid::Edid;
use crate::timing::Timing;

fn wh(t: Option<Timing>) -> Option<(u32, u32, u32)> {
    t.map(|t| (t.width, t.height, t.refresh_mhz / 1000))
}

#[test]
fn each_aspect_code_scales_the_width() {
    // 1920 = (209 + 31) * 8; 1280 = (129 + 31) * 8.
    assert_eq!(wh(standard::decode(209, 0x00, true)), Some((1920, 1200, 60)));
    assert_eq!(wh(standard::decode(129, 0x40 | 15, true)), Some((1280, 960, 75)));
    assert_eq!(wh(standard::decode(129, 0x80, true)), Some((1280, 1024, 60)));
    assert_eq!(wh(standard::decode(209, 0xC0, true)), Some((1920, 1080, 60)));
    assert_eq!(wh(standard::decode(129, 0x00, false)), Some((1280, 1280, 60)), "1:1 before 1.3");
}

#[test]
fn unused_and_reserved_slots_name_nothing() {
    assert!(standard::decode(1, 1, true).is_none());
    assert!(standard::decode(0, 0x40, true).is_none());
}

#[test]
fn base_slots_and_an_fa_descriptor_both_count() {
    let mut b = base();
    b[38] = 209;
    b[39] = 0xC0;
    seal(&mut b);
    let mut body = [0x01u8; 13];
    body[0] = 129;
    body[1] = 0x80;
    body[12] = 0x0A;
    put(&mut b, 3, &display(0xFA, &body));
    let mut v = Vec::new();
    Edid::parse(&b).unwrap().for_each_timing(&mut |t| v.push((t.width, t.height)));
    assert_eq!(v, [(1920, 1080), (1280, 1024)]);
}

#[test]
fn a_cvt_code_offers_each_rate_bit_it_sets() {
    // 1080 lines: (539 + 1) * 2; 539 = 0x21B. 16:9; 60 Hz and 60 RB.
    let mut v = Vec::new();
    cvt3::code(&[0x1B, 0x20 | 0x04, 0x20 | 0x08 | 0x01], &mut |t| v.push(t));
    let got: Vec<_> =
        v.iter().map(|t| (t.width, t.height, t.refresh_mhz, t.reduced_blanking)).collect();
    assert_eq!(got, [(1920, 1080, 60_000, false), (1920, 1080, 60_000, true)]);
    v.clear();
    cvt3::code(&[0, 0, 0], &mut |t| v.push(t));
    assert!(v.is_empty(), "an empty slot");
    // 768 lines at 16:10 cut to a multiple of 8: 1228.8 -> 1224.
    cvt3::code(&[0x7F, 0x18, 0x10], &mut |t| v.push(t));
    assert_eq!((v[0].width, v[0].height, v[0].refresh_mhz), (1224, 768, 50_000));
}
