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

//! A DMA32 map is placed below 4 GiB or refused, never placed high (D6b).

use super::placement::fits_below_4g;
use super::sources::{sources, Source};

const LOW: u64 = 0x8000_0000;
const HIGH: u64 = 0x1_2000_0000;

/// alloc.rs's take over a machine where only `free` sources have frames:
/// the low ones give LOW, the others HIGH.
fn take(high: bool, dma32: bool, free: &[Source]) -> Option<u64> {
    sources(high, dma32).iter().find_map(|s| {
        let below = matches!(s, Source::LowPool | Source::Below4G);
        free.contains(s).then_some(if below { LOW } else { HIGH })
    })
}

#[test]
fn a_dma32_map_looks_only_below_4g() {
    assert_eq!(sources(false, true), [Source::LowPool, Source::Below4G]);
}

#[test]
fn a_dma32_map_is_placed_low() {
    let all = [Source::LowPool, Source::Below4G, Source::Anywhere, Source::HighFirst];
    assert_eq!(take(false, true, &all), Some(LOW));
    assert_eq!(take(false, true, &[Source::Below4G, Source::Anywhere]), Some(LOW), "pool spent");
    assert!(fits_below_4g(LOW, 1));
}

#[test]
fn a_dma32_map_is_refused_when_low_memory_is_spent() {
    let high_only = [Source::Anywhere, Source::HighFirst, Source::DisplayPool];
    assert_eq!(take(false, true, &high_only), None);
    // A map with no flag takes the high frames; one for a 32-bit device never does.
    assert_eq!(take(false, false, &high_only), Some(HIGH));
    assert!(!fits_below_4g(HIGH, 1));
}
