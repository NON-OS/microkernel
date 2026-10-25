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

//! The narrowed fallback (L79) over whole tables of records: a framebuffer
//! no BAR holds, in RAM, goes to the one display function there is, and to
//! nobody when there are two or none; once some BAR holds it, the fallback
//! is off. Addresses are illustrative.

use super::cover_find::{find, Rec, KIB, MIB, MMIO};
use super::cover_tests::SMALL;

/// A frame in RAM, below every BAR here.
const RAM: (u64, u64) = (0x7E00_0000, SMALL);

fn display(base: u64) -> Rec {
    Rec::new(&[(base, 16 * MIB, MMIO), (base + 32 * MIB, 4 * KIB, MMIO)], true)
}

#[test]
fn a_ram_frame_falls_to_the_lone_display() {
    let nic = Rec::new(&[(0xFEB0_0000, 128 * KIB, MMIO)], false);
    assert_eq!(find(RAM, &[nic, display(0xFC00_0000)]), vec![1]);
    assert_eq!(find(RAM, &[display(0xFC00_0000)]), vec![0], "the only record");
}

#[test]
fn a_ram_frame_with_two_displays_or_none_names_nobody() {
    let nic = Rec::new(&[(0xFEB0_0000, 128 * KIB, MMIO)], false);
    let two = [display(0xC000_0000), nic, display(0xD000_0000)];
    assert!(find(RAM, &two).is_empty(), "two displays");
    let nic = Rec::new(&[(0xFEB0_0000, 128 * KIB, MMIO)], false);
    assert!(find(RAM, &[nic]).is_empty(), "no display");
    assert!(find(RAM, &[]).is_empty(), "no record");
}

#[test]
fn a_frame_some_bar_holds_turns_the_fallback_off() {
    let holder = Rec::new(&[(0x7E00_0000, 16 * MIB, MMIO)], false);
    assert_eq!(find(RAM, &[display(0xFC00_0000), holder]), vec![1], "the lone display");
}
