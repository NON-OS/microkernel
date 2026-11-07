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

//! The cache's budget: least recently used first, never an installed app's
//! image, never the one just used, and nothing dropped once it fits.

use crate::install::image::prune::{budget, evict, read_stamp, stamp, Cached};

const MIB: u64 = 1 << 20;

fn c(n: u8, mib: u64, used: u64, installed: bool) -> Cached {
    Cached { pin: [n; 32], bytes: mib * MIB, used, installed }
}

#[test]
fn the_least_recently_used_goes_first_until_it_fits() {
    let cache = [c(1, 300, 30, false), c(2, 300, 10, false), c(3, 300, 20, false)];
    assert_eq!(evict(&cache, 600 * MIB, &[9; 32]), [[2; 32]]);
    assert_eq!(evict(&cache, 300 * MIB, &[9; 32]), [[2; 32], [3; 32]]);
    assert!(evict(&cache, 900 * MIB, &[9; 32]).is_empty(), "nothing dropped when it fits");
}

#[test]
fn an_installed_apps_image_and_the_one_just_used_are_never_dropped() {
    let cache = [c(1, 500, 1, true), c(2, 500, 2, false), c(3, 500, 3, false)];
    assert_eq!(evict(&cache, 0, &[3; 32]), [[2; 32]], "even past the budget");
    let all_kept = [c(1, 900, 1, true), c(2, 900, 2, true)];
    assert!(evict(&all_kept, 100 * MIB, &[9; 32]).is_empty());
}

#[test]
fn an_unstamped_image_counts_as_oldest() {
    let cache = [c(1, 500, 0, false), c(2, 500, 5, false)];
    assert_eq!(evict(&cache, 500 * MIB, &[9; 32]), [[1; 32]]);
}

#[test]
fn the_stamp_and_the_budget_read_back() {
    assert_eq!(read_stamp(&stamp(1_792_844_669_000)), 1_792_844_669_000);
    assert_eq!(read_stamp(b"short"), 0);
    assert_eq!(budget(None), 2048 * MIB);
    assert_eq!(budget(Some(b"4096\n")), 4096 * MIB);
    assert_eq!(budget(Some(b"1")), 256 * MIB, "a floor");
    assert_eq!(budget(Some(b"99999999")), (64 << 10) * MIB, "a ceiling");
    assert_eq!(budget(Some(b"lots")), 2048 * MIB);
}
