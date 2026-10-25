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

//! The DMT table, its CVT rows held to the CVT arithmetic.

use crate::cvt::{cvt, Blanking};
use crate::dmt::{all, dmt, Kind};

#[test]
fn the_file_gives_every_row_once() {
    assert_eq!(all().len(), 86);
    let mut ids: Vec<u8> = all().iter().map(|m| m.id).collect();
    ids.sort();
    ids.dedup();
    assert_eq!(ids.len(), all().len(), "an ID listed twice");
}

#[test]
fn every_cvt_row_is_exactly_what_cvt_computes() {
    let mut checked = 0;
    for m in all() {
        let b = match m.kind {
            Kind::Own => continue,
            Kind::Cvt => Blanking::Standard,
            Kind::Reduced => Blanking::Reduced,
            Kind::ReducedV2 => Blanking::ReducedV2,
        };
        let t = m.timing;
        assert_eq!(cvt(t.h_active, t.v_active, m.hz * 1000, b), Some(t), "DMT {:02X}", m.id);
        checked += 1;
    }
    assert_eq!(checked, 45);
}

#[test]
fn every_row_runs_near_its_nominal_rate() {
    // Two old modes run off their names: 640x480 at 72.809 Hz and 800x600
    // at 60.317 Hz, as the DMT gives them. 1.5 percent holds them both.
    for m in all() {
        let r = m.timing.refresh_mhz() as i64;
        let want = m.hz as i64 * 1000;
        assert!((r - want).abs() * 1000 < want * 15, "DMT {:02X} runs at {r}", m.id);
    }
}

#[test]
fn lookups_find_the_mode_with_the_blanking_asked_for() {
    let m = dmt(1920, 1080, 60, false).unwrap();
    assert_eq!((m.id, m.timing.pixel_khz, m.timing.h_total()), (0x52, 148_500, 2200));
    let m = dmt(1920, 1200, 60, true).unwrap();
    assert_eq!((m.id, m.timing.pixel_khz), (0x44, 154_000));
    assert_eq!(dmt(1920, 1200, 60, false).unwrap().id, 0x45);
    assert_eq!(dmt(4096, 2160, 60, true).unwrap().kind, Kind::ReducedV2);
    assert!(dmt(1921, 1080, 60, false).is_none());
    assert!(dmt(1920, 1080, 60, true).is_none(), "the DMT has no RB 1080p");
}
