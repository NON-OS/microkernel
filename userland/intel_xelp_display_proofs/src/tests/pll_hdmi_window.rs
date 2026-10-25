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
//! Every HDMI symbol clock in kHz: the DCO window is pinned from inside
//! (narrowing it by 1 MHz changes some answer), and widening it changes
//! none, because a DCO just outside is never the nearest to 8,999 MHz.
//! So mutants that widen it are equivalent; those that narrow it are not.

use nonos_intel_xelp_display::pll::hdmi::LIST;

fn pick(s: u32, min: u64, max: u64) -> Option<u32> {
    let afe = 5 * s as u64;
    let off = |d: u32| (afe * d as u64).abs_diff(8_999_000);
    let mut best: Option<u32> = None;
    for d in LIST.into_iter().filter(|d| (min..=max).contains(&(afe * *d as u64))) {
        if best.is_none_or(|b| off(d) < off(b)) {
            best = Some(d);
        }
    }
    best
}

#[test]
fn window_pinned_from_inside_and_wide_enough() {
    let (mut narrowed, mut widened) = (0, 0);
    for s in (20_000..=600_000).filter(|s| !(500_000 < *s && *s < 533_200)) {
        let real = pick(s, 7_998_000, 10_000_000);
        let wide = [pick(s, 7_990_000, 10_000_000), pick(s, 7_998_000, 10_010_000)];
        let narrow = [pick(s, 7_999_000, 10_000_000), pick(s, 7_998_000, 9_999_000)];
        widened += wide.iter().filter(|w| **w != real).count();
        narrowed += narrow.iter().filter(|n| **n != real).count();
    }
    assert_eq!(widened, 0);
    assert!(narrowed > 0);
}
