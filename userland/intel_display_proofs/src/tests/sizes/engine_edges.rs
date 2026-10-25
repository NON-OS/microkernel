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

//! GGTT placement, watermarks and the backlight at every edge.

use super::Pick;
use crate::engine::backlight::{get, set, BLC_PWM_PCH_CTL2};
use crate::engine::ggtt_range::pick;
use crate::engine::wm::levels;
use crate::model::Gen9;

#[test]
fn a_ggtt_run_is_aligned_inside_and_clear_of_firmware_at_every_edge() {
    let mut p = Pick(0x7000_6677);
    let mut placed = 0;
    for _ in 0..300_000 {
        let (total, pages, align, avoid) = (p.edge(), p.edge(), p.edge(), (p.edge(), p.edge()));
        let Some(s) = pick(total, pages, align, avoid) else { continue };
        let end = s as u64 + pages as u64;
        assert!(s % align == 0 && end <= total as u64, "{total} {pages} {align} {avoid:?}");
        assert!(end <= avoid.0 as u64 || s >= avoid.1, "{total} {pages} {align} {avoid:?}");
        placed += 1;
    }
    assert!(placed > 1_000, "{placed}");
}

#[test]
fn watermark_levels_stay_in_their_fields_at_every_edge() {
    let mut p = Pick(0x7000_3317);
    for _ in 0..100_000 {
        let (lat, share) = ([(); 8].map(|_| p.edge()), p.edge());
        for l in levels(&lat, p.edge(), p.edge(), p.edge(), share) {
            assert!(!l.enabled || (l.lines <= 31 && l.blocks < share), "{l:?}");
        }
    }
}

#[test]
fn the_backlight_reads_and_sets_within_its_period_at_every_edge() {
    let mut p = Pick(0x7000_B117);
    for _ in 0..50_000 {
        let mut m = Gen9::new(1);
        m.set(BLC_PWM_PCH_CTL2, p.edge());
        let max = p.edge();
        assert!(get(&mut m, max) <= max);
        let level = p.edge();
        if set(&mut m, level, max).is_ok() {
            let period = m.get(BLC_PWM_PCH_CTL2) >> 16;
            assert!(m.get(BLC_PWM_PCH_CTL2) & 0xFFFF <= period.max(1) && level <= max);
        }
    }
}
