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

//! Bring-up's table from a well-formed GET_DISPLAY_INFO with any sizes in
//! it: each enabled scanout with a size recorded at its own mode when that
//! is at least 16x16 and its frame fits the display grant (32 MiB, four
//! bytes a pixel),
//! else at 1280x720, and scanout 0 at 1280x720 when none was enabled.

use super::device::case;
use super::device::Reply;
use super::reply;
use crate::setup::scanouts::seed;
use crate::state::{FenceCounter, ScanoutTable};
use crate::tests::wire_sweep::rng::Rng;

const GRANT_BYTES: u64 = 32 << 20;

fn expected(w: u32, h: u32) -> (u32, u32) {
    let bytes = u128::from(w) * u128::from(h) * 4;
    let fits = w >= 16 && h >= 16 && bytes <= u128::from(GRANT_BYTES);
    if fits {
        (w, h)
    } else {
        (1280, 720)
    }
}

#[test]
fn any_display_sizes_seed_a_table_of_modes_the_grant_holds() {
    let mut r = Rng(0x7365_6501);
    for i in 0..2_000 {
        let bytes = reply::display(&mut r);
        let entries = super::oracle::display(&Reply { bytes: bytes.clone(), claimed: None });
        let entries = entries.expect("well formed");
        let table = ScanoutTable::new();
        let (got, _, live) =
            case(Reply { bytes, claimed: None }, |q| seed(q, &table, &FenceCounter::new()));
        assert!(got.is_ok() && live, "case {i}: seeding refused");
        let on: Vec<_> = (0..16)
            .filter(|&s| entries[s].2 != 0 && entries[s].0 != 0 && entries[s].1 != 0)
            .collect();
        for (s, &(w, h, _)) in entries.iter().enumerate() {
            let t = table.get(s as u32).unwrap();
            if on.contains(&s) {
                assert!(
                    t.enabled && (t.width, t.height) == expected(w, h),
                    "case {i}: scanout {s}"
                );
            } else if !(s == 0 && on.is_empty()) {
                assert!(!t.enabled, "case {i}: scanout {s} recorded though off");
            }
        }
        if on.is_empty() {
            let t = table.get(0).unwrap();
            assert!(t.enabled && (t.width, t.height) == (1280, 720), "case {i}: no default");
        }
    }
}
