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

//! DisplayID 1.3 sections: type I detailed timings in 10 kHz, type III
//! short timings by width and aspect, type IV DMT IDs from the shared DMT
//! table, and the native size and refresh range a 1.3 panel states.

use crate::build::base;
use crate::build_did::{block, displayid_v, type7};
use crate::build_ext::with;
use crate::edid::Edid;
use crate::timing::Timing;

fn timings(raw: &[u8]) -> Vec<Timing> {
    let mut v = Vec::new();
    Edid::parse(raw).unwrap().for_each_timing(&mut |t| v.push(t));
    v
}

#[test]
fn a_type_i_timing_reads_its_clock_in_tens_of_kilohertz() {
    // Type VII's layout with the clock stored in 10 kHz: 14850 means 148.5 MHz.
    let d = type7(14_850, [1920, 280, 88, 44, 1080, 45, 4, 5], 0x80);
    let raw = with(base(), &[displayid_v(0x13, &block(0x03, 0, &d))]);
    let p = Edid::parse(&raw).unwrap().preferred().unwrap();
    assert_eq!((p.pixel_khz, p.h_active, p.v_active), (148_500, 1920, 1080));
    let t = timings(&raw);
    assert_eq!((t[0].refresh_mhz / 1000, t[0].preferred), (60, true));
}

#[test]
fn type_iii_and_type_iv_name_sizes_and_dmt_modes() {
    // 1920 wide (239 = 1920 / 8 - 1) at 16:10, 60 Hz, reduced blanking; then DMT 0x52 (1920x1080 at 60).
    let mut blocks = block(0x05, 0, &[0x15, 239, 59]);
    blocks.extend(block(0x06, 0, &[0x52, 0xEE]));
    let t = timings(&with(base(), &[displayid_v(0x13, &blocks)]));
    let sizes: Vec<_> =
        t.iter().map(|t| (t.width, t.height, t.refresh_mhz / 1000, t.reduced_blanking)).collect();
    assert_eq!(sizes, [(1920, 1200, 60, true), (1920, 1080, 60, false)], "0xEE is no DMT ID");
}

#[test]
fn a_panel_states_its_native_size_and_refresh_range() {
    let mut params = [0u8; 12];
    params[..2].copy_from_slice(&3440u16.to_le_bytes()); // 344.0 mm
    params[2..4].copy_from_slice(&1940u16.to_le_bytes());
    params[4..6].copy_from_slice(&5120u16.to_le_bytes());
    params[6..8].copy_from_slice(&2880u16.to_le_bytes());
    let mut range = [0u8; 15];
    (range[10], range[11]) = (48, 120);
    let mut blocks = block(0x01, 0, &params);
    blocks.extend(block(0x09, 0, &range));
    let raw = with(base(), &[displayid_v(0x13, &blocks)]);
    let e = Edid::parse(&raw).unwrap();
    let n = e.displayid_native().unwrap();
    assert_eq!((n.width, n.height, n.width_mm, n.height_mm), (5120, 2880, 344, 194));
    assert_eq!(e.displayid_refresh(), Some((48, 120)));
}
