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

//! The capability list walk against lists no conforming function presents.

use super::space::{qemu_modern, Space, MODERN_NET};
use crate::caps::{parse, CapWalk, CFG_COMMON, MAX_CAPS};

fn walk(s: &Space) -> Vec<usize> {
    CapWalk::new(&s.cfg()).collect()
}

#[test]
fn the_qemu_list_is_walked_in_list_order() {
    assert_eq!(walk(&qemu_modern(MODERN_NET)), vec![0x98, 0x84, 0x70, 0x60, 0x50, 0x40]);
}

#[test]
fn no_capabilities_list_bit_means_no_walk() {
    let s = qemu_modern(MODERN_NET).without_cap_list();
    assert!(walk(&s).is_empty());
    assert_eq!(parse(&s.cfg(), &super::space::qemu_modern_bars()).common, None);
}

#[test]
fn a_pointer_into_the_standard_header_ends_the_walk() {
    // Head below 0x40.
    let mut s = Space::new(MODERN_NET, 0x20);
    s.cap(0x20, 0x00, CFG_COMMON, 4, 0, 0x1000);
    assert!(walk(&s).is_empty());
    // A later pointer below 0x40: the walk stops there, after what it saw.
    let mut s = Space::new(MODERN_NET, 0x40);
    s.cap(0x40, 0x3C, CFG_COMMON, 4, 0, 0x1000);
    assert_eq!(walk(&s), vec![0x40]);
}

#[test]
fn reserved_pointer_bits_are_masked() {
    let mut s = Space::new(MODERN_NET, 0x43);
    s.cap(0x40, 0x53, CFG_COMMON, 4, 0, 0x1000);
    s.cap(0x50, 0x00, CFG_COMMON, 4, 0, 0x1000);
    assert_eq!(walk(&s), vec![0x40, 0x50]);
}

#[test]
fn a_list_that_loops_back_is_walked_once() {
    let mut s = Space::new(MODERN_NET, 0x40);
    s.cap(0x40, 0x50, CFG_COMMON, 4, 0, 0x1000);
    s.cap(0x50, 0x60, CFG_COMMON, 4, 0, 0x1000);
    s.cap(0x60, 0x40, CFG_COMMON, 4, 0, 0x1000);
    assert_eq!(walk(&s), vec![0x40, 0x50, 0x60]);
}

#[test]
fn a_capability_pointing_at_itself_is_visited_once() {
    let mut s = Space::new(MODERN_NET, 0x40);
    s.cap(0x40, 0x40, CFG_COMMON, 4, 0, 0x1000);
    assert_eq!(walk(&s), vec![0x40]);
}

#[test]
fn the_walk_is_bounded_by_the_capability_area() {
    // A chain through every dword from 0x40 to 0xFC: 48 entries, then the
    // last points back at the first.
    let mut s = Space::new(MODERN_NET, 0x40);
    for at in (0x40u16..=0xFC).step_by(4) {
        let next = if at == 0xFC { 0x40 } else { at + 4 };
        s.put8(at as usize, 0x01).put8(at as usize + 1, next as u8);
    }
    let seen = walk(&s);
    assert_eq!(seen.len(), MAX_CAPS);
    assert_eq!(MAX_CAPS, 48);
}

#[test]
fn every_byte_pattern_terminates_inside_the_bound() {
    // Arbitrary config spaces, capability bit forced on: the walk always
    // ends within MAX_CAPS steps and only yields offsets it can read.
    let mut state = 0x9E37_79B9_7F4A_7C15u64;
    for _ in 0..20_000 {
        let mut bytes = [0u8; 256];
        for b in bytes.iter_mut() {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            *b = state as u8;
        }
        bytes[0x06] |= 0x10;
        let cfg = crate::pci::ConfigSpace::from_bytes(bytes);
        let seen: Vec<usize> = CapWalk::new(&cfg).collect();
        assert!(seen.len() <= MAX_CAPS);
        assert!(seen.iter().all(|&p| (0x40..=0xFC).contains(&p) && p % 4 == 0));
        let mut unique = seen.clone();
        unique.sort_unstable();
        unique.dedup();
        assert_eq!(unique.len(), seen.len(), "a capability was visited twice");
        let _ = parse(&cfg, &super::space::qemu_modern_bars());
    }
}
