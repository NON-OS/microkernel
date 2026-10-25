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

//! ASLR at the ends of the address space: a slide that would carry an
//! address below zero or past the top is refused, never wrapped.

use crate::elf::aslr::AslrManager;
use crate::stand_in::sources;

#[test]
fn a_stack_too_low_to_slide_is_refused() {
    sources(true, true, false);
    let mut m = AslrManager::new();
    // A slide is up to 16 MiB; from a stack at 4 KiB most would wrap.
    let tries: Vec<_> = (0..16).map(|_| m.randomize_stack(0x1000)).collect();
    assert!(tries.iter().all(|s| s.is_none_or(|s| s <= 0x1000)), "{tries:x?}");
    assert!(tries.iter().any(Option::is_none));
}

#[test]
fn a_base_or_heap_too_high_to_slide_is_refused() {
    sources(true, true, false);
    let mut m = AslrManager::new();
    let top = u64::MAX - 0xfff;
    let above = |a: Option<u64>| a.is_none_or(|a| a >= top & !0xfff);
    for _ in 0..16 {
        let (b, h) = (m.randomize_base(top), m.randomize_heap(top));
        assert!(above(b) && above(h), "{b:x?} {h:x?}");
    }
    assert_eq!(m.randomize_base(top).or(m.randomize_heap(top)), None);
}
