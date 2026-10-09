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

use super::placement::{fits_below_4g, low32_may_take, DMA32_CEILING};

const PAGE: u64 = 4096;

#[test]
fn a_run_ending_exactly_at_4g_fits() {
    assert!(fits_below_4g(DMA32_CEILING - 2 * PAGE, 2));
    assert!(fits_below_4g(0x100000, 1));
}

#[test]
fn a_run_crossing_or_above_4g_does_not() {
    assert!(!fits_below_4g(DMA32_CEILING - PAGE, 2));
    assert!(!fits_below_4g(DMA32_CEILING, 1));
    assert!(!fits_below_4g(0x2_4000_0000, 2));
}

#[test]
fn overflow_is_refused_not_wrapped() {
    assert!(!fits_below_4g(u64::MAX - PAGE, 2));
    assert!(!fits_below_4g(0, u64::MAX));
}

/* The low pool is 2048 pages with a 1024-page floor. */
const POOL: usize = 2048;
const FLOOR: usize = 1024;

#[test]
fn a_map_for_a_64bit_device_stops_at_the_floor() {
    assert!(low32_may_take(POOL, 64, POOL, FLOOR, false));
    assert!(low32_may_take(FLOOR + 64, 64, POOL, FLOOR, false));
    assert!(!low32_may_take(FLOOR + 63, 64, POOL, FLOOR, false));
    assert!(!low32_may_take(FLOOR, 1, POOL, FLOOR, false));
}

#[test]
fn a_map_for_a_32bit_device_may_take_the_floor() {
    assert!(low32_may_take(FLOOR, 64, POOL, FLOOR, true));
    assert!(low32_may_take(2, 2, POOL, FLOOR, true));
    assert!(!low32_may_take(1, 2, POOL, FLOOR, true));
}

#[test]
fn every_other_driver_together_cannot_starve_the_wifi_card() {
    // xHCI, HDA, NVMe and the rest spawn first and map without the flag; the
    // rtl8821ce's 69 pages must still be there when it maps.
    let mut free = POOL;
    while low32_may_take(free, 1, POOL, FLOOR, false) {
        free -= 1;
    }
    assert_eq!(free, FLOOR);
    let mut wifi = 0;
    for pages in [2usize, 1, 1, 32, 1, 32] {
        assert!(low32_may_take(free, pages, POOL, FLOOR, true));
        free -= pages;
        wifi += pages;
    }
    assert_eq!(wifi, 69);
}

#[test]
fn a_pool_smaller_than_the_floor_is_reserved_whole() {
    assert!(!low32_may_take(512, 1, 512, FLOOR, false));
    assert!(low32_may_take(512, 1, 512, FLOOR, true));
}

#[test]
fn an_empty_or_short_pool_gives_nothing() {
    assert!(!low32_may_take(0, 1, POOL, FLOOR, true));
    assert!(!low32_may_take(POOL, 0, POOL, FLOOR, true));
}
