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
//! The HDMI search at its edges: a DCO one kHz either side of 7,998 and
//! 10,000 MHz for every divider, and two facts about the whole range that
//! make two of the PRM's rules unobservable on kHz inputs, shown here so
//! that their mutants are known to be equivalent, not missed.

use nonos_intel_xelp_display::pll::hdmi::{dividers, LIST};

const MIN: u64 = 7_998_000;
const MAX: u64 = 10_000_000;

fn in_range(afe: u64, d: u32) -> bool {
    (MIN..=MAX).contains(&(afe * d as u64))
}

#[test]
fn dco_bounds_are_inclusive_and_exact() {
    for d in LIST {
        let at_min = MIN.div_ceil(5 * d as u64) as u32;
        let at_max = (MAX / (5 * d as u64)) as u32;
        for s in [at_min - 1, at_min, at_max, at_max + 1] {
            let afe = 5 * s as u64;
            let any = LIST.iter().any(|x| in_range(afe, *x));
            let ok = any && (20_000..=600_000).contains(&s) && !(s > 500_000 && s < 533_200);
            assert_eq!(dividers(s, 24_000).is_ok(), ok, "divider {d}, symbol {s}");
            if let Ok(r) = dividers(s, 24_000) {
                assert!(in_range(afe, r.p * r.q * r.k), "{s}");
            }
        }
    }
}

#[test]
fn no_divider_reaches_the_gap() {
    // Vol 12 excludes 500 to 533.2 MHz (p. 170); no listed divider puts a
    // DCO in range there either, so the two rules agree.
    for s in 500_001u64..533_200 {
        assert!(!LIST.iter().any(|d| in_range(5 * s, *d)), "{s}");
    }
}

#[test]
fn no_exact_tie_in_khz() {
    // Two dividers tie when their DCOs sit equally either side of 8,999
    // MHz. On whole kHz, whenever two in range do, a third lies nearer, so
    // "first listed wins" never decides.
    for (i, a) in LIST.iter().enumerate() {
        for b in &LIST[i + 1..] {
            let sum = 5 * (a + b) as u64;
            if 17_998_000 % sum == 0 {
                let afe = 5 * (17_998_000 / sum);
                let off = |d: u32| (afe * d as u64).abs_diff(8_999_000);
                let near = LIST.iter().filter(|d| in_range(afe, **d)).map(|d| off(*d)).min();
                let both = in_range(afe, *a) && in_range(afe, *b);
                assert!(!both || near < Some(off(*a)), "{a} {b}");
            }
        }
    }
}
