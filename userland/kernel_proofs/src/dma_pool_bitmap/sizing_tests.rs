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

use super::sizing::{low32_fit, low32_floor, low32_target_pages, pressure_quarter};

const MIB: u64 = 1 << 20;

#[test]
fn the_pool_grows_with_low_memory_between_8_and_32_mib() {
    assert_eq!(low32_target_pages(0), 2048);
    assert_eq!(low32_target_pages(512 * MIB), 2048);
    assert_eq!(low32_target_pages(1024 * MIB), 2048);
    assert_eq!(low32_target_pages(2048 * MIB), 4096);
    assert_eq!(low32_target_pages(3 * 1024 * MIB), 6144);
    assert_eq!(low32_target_pages(4096 * MIB), 8192);
    assert_eq!(low32_target_pages(u64::MAX), 8192);
}

#[test]
fn a_short_region_gives_what_it_holds_or_nothing() {
    assert_eq!(low32_fit(4096, 10_000), 4096);
    assert_eq!(low32_fit(4096, 1500), 1500);
    assert_eq!(low32_fit(4096, 256), 256);
    assert_eq!(low32_fit(4096, 255), 0);
    assert_eq!(low32_fit(4096, 0), 0);
}

#[test]
fn the_floor_is_half_the_pool_as_the_fixed_one_was() {
    assert_eq!(low32_floor(2048), 1024);
    assert_eq!(low32_floor(8192), 4096);
    assert_eq!(low32_floor(257), 128);
}

#[test]
fn pressure_is_counted_in_quarters_and_never_overflows() {
    assert_eq!(pressure_quarter(0, 2048), 0);
    assert_eq!(pressure_quarter(1535, 2048), 2);
    assert_eq!(pressure_quarter(1536, 2048), 3);
    assert_eq!(pressure_quarter(2048, 2048), 4);
    assert_eq!(pressure_quarter(usize::MAX, 2048), 4);
    assert_eq!(pressure_quarter(5, 0), 0);
}
