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

use super::cache::{cache_for, known, Cache, Refused, FLAGS_KNOWN, MAP_WC, PREFETCH};

/// A 16 MiB prefetchable framebuffer BAR, and a page of registers.
const FB: (u64, u64, u8) = (0xE000_0000, 0x0100_0000, PREFETCH);
const REGS: (u64, u64, u8) = (0xFEB0_0000, 0x1000, 0);
const PAGE: u64 = 0x1000;

#[test]
fn without_the_flag_every_bar_is_uncached() {
    for (bar, ready) in [(FB, true), (FB, false), (REGS, true), (REGS, false)] {
        assert_eq!(cache_for(0, bar, (bar.0, PAGE), ready), Ok(Cache::Uncached));
    }
}

#[test]
fn a_prefetchable_bar_is_write_combined_once_every_pat_has_the_entry() {
    assert_eq!(cache_for(MAP_WC, FB, (FB.0, FB.1), true), Ok(Cache::WriteCombined));
    let last = (FB.0 + FB.1 - PAGE, PAGE);
    assert_eq!(cache_for(MAP_WC, FB, last, true), Ok(Cache::WriteCombined));
}

#[test]
fn without_the_pat_entry_write_combining_falls_back_to_uncached() {
    assert_eq!(cache_for(MAP_WC, FB, (FB.0, PAGE), false), Ok(Cache::Uncached));
}

#[test]
fn registers_are_never_write_combined() {
    for ready in [true, false] {
        let r = cache_for(MAP_WC, REGS, (REGS.0, PAGE), ready);
        assert_eq!(r, Err(Refused::NotPrefetchable));
    }
}

#[test]
fn pages_reaching_past_the_bar_are_refused() {
    let end = FB.0 + FB.1;
    for pages in [(end - PAGE, 2 * PAGE), (FB.0 - PAGE, 2 * PAGE), (u64::MAX - 0xFFF, 2 * PAGE)] {
        assert_eq!(cache_for(MAP_WC, FB, pages, true), Err(Refused::PastBar));
    }
    // A BAR smaller than a page: the page it sits in reaches past it.
    let small = (0xE000_0800, 0x800, PREFETCH);
    assert_eq!(cache_for(MAP_WC, small, (0xE000_0000, PAGE), true), Err(Refused::PastBar));
    let wraps = (u64::MAX - 0xFFF, 0x2000, PREFETCH);
    assert_eq!(cache_for(MAP_WC, wraps, (u64::MAX - 0xFFF, PAGE), true), Err(Refused::PastBar));
}

#[test]
fn an_unknown_bit_is_refused_before_anything_else() {
    assert_eq!(FLAGS_KNOWN, MAP_WC);
    for bit in 1..32 {
        let f = 1u32 << bit;
        assert!(!known(f));
        assert_eq!(cache_for(f, FB, (FB.0, PAGE), true), Err(Refused::Unknown));
        assert_eq!(cache_for(f | MAP_WC, REGS, (REGS.0, PAGE), true), Err(Refused::Unknown));
    }
}
