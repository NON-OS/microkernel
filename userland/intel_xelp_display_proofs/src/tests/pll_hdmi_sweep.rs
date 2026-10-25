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
//! Every HDMI symbol clock on a fine step, each answer re-derived here
//! from Vol 12's rule, divider by divider.

use nonos_intel_xelp_display::pll::hdmi::dividers;

/// Vol 12's divider list, copied here from p. 181 on its own.
const LIST: [u32; 46] = [
    2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 24, 28, 30, 32, 36, 40, 42, 44, 48, 50, 52, 54, 56, 60, 64,
    66, 68, 70, 72, 76, 78, 80, 84, 88, 90, 92, 96, 98, 100, 102, 3, 5, 7, 9, 15, 21,
];

#[test]
fn sweep_matches_the_rule() {
    let mut taken = 0;
    for ref_khz in [24_000u64, 19_200] {
        for s in (19_000u32..601_000).step_by(37) {
            let afe = 5 * s as u64;
            // The rule restated: in-range DCOs, nearest 8999 MHz, first wins.
            let mut want: Option<u32> = None;
            let off = |d: u32| (afe * d as u64).abs_diff(8_999_000);
            for d in
                LIST.into_iter().filter(|d| (7_998_000..=10_000_000).contains(&(afe * *d as u64)))
            {
                if want.is_none_or(|w| off(d) < off(w)) {
                    want = Some(d);
                }
            }
            let gap = s > 500_000 && s < 533_200;
            let got = dividers(s, ref_khz as u32);
            let (Some(div), false, true) = (want, gap, (20_000..=600_000).contains(&s)) else {
                assert!(got.is_err(), "{s}");
                continue;
            };
            let d = got.unwrap();
            taken += 1;
            assert_eq!(d.p * d.q * d.k, div, "{s}");
            assert!(d.q == 1 || d.k == 2);
            assert!(d.cfgcr0().is_some() && d.cfgcr1().is_some());
            // The DCO given back is the one wanted, to within the fraction's
            // last step.
            let dco = (((d.integer as u64) << 15) + d.fraction as u64) * ref_khz;
            let exact = (afe * div as u64) << 15;
            assert!(exact >= dco && exact - dco < ref_khz, "{s}");
        }
    }
    assert!(taken > 20_000, "{taken}");
}
