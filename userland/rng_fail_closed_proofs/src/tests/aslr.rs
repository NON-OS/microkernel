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

//! ASLR takes the CPU's random instruction, then the seeded RNG, and when
//! neither answers it refuses: no layout is made from a constant.

use crate::elf::aslr::AslrManager;
use crate::stand_in::{sources, CPU_TAG, SEEDED_TAG};

#[test]
fn nothing_answers_so_nothing_is_placed() {
    sources(false, true, false);
    let mut m = AslrManager::new();
    assert_eq!(m.random_offset(0x1000), None);
    assert_eq!(m.randomize_base(0x40_0000), None);
    assert_eq!(m.randomize_stack(0x7fff_0000_0000), None);
    assert_eq!(m.randomize_heap(0x1000_0000), None);
    assert_eq!(AslrManager::random_address(0x1000, 0x10_0000, 0x1000), None);
    assert!(!m.reseed());
    assert_eq!(m.entropy(), 0, "a refused draw changes nothing");
}

#[test]
fn turned_off_randomisation_needs_no_draw() {
    sources(false, true, false);
    let mut m = AslrManager::with_settings(false, false, false);
    assert_eq!(m.randomize_base(0x40_0000), Some(0x40_0000));
    assert_eq!(m.randomize_stack(0x7000), Some(0x7000));
    assert_eq!(m.randomize_heap(0x9000), Some(0x9000));
    assert_eq!(m.random_offset(0), Some(0));
}

#[test]
fn the_cpu_answers_first_then_the_seeded_rng() {
    sources(true, true, true);
    let mut m = AslrManager::disabled();
    assert!(m.reseed());
    assert_eq!(m.entropy() >> 56, CPU_TAG);
    sources(true, true, false);
    let mut m = AslrManager::disabled();
    assert!(m.reseed());
    assert_eq!(m.entropy() >> 56, SEEDED_TAG);
}

#[test]
fn a_working_source_places_pages() {
    sources(true, true, false);
    let mut m = AslrManager::new();
    let base = m.randomize_base(0x40_0000).expect("placed");
    assert_eq!(base & 0xfff, 0);
    assert!((0x40_0000..0x40_0000 + 0x4000_0000).contains(&base));
    let stack = m.randomize_stack(0x7fff_0000_0000).expect("placed");
    assert!(stack <= 0x7fff_0000_0000 && stack & 0xfff == 0);
    let a = AslrManager::random_address(0x1000, 0x10_0000, 0x1000).expect("placed");
    assert!((0x1000..0x10_0000).contains(&a));
}
