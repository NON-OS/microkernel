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

//! The second layout read: the subsections for this ROM, by priority, with
//! sections Realtek has not defined passed over as Linux's driver does.

use super::fixture_rtbt::rtbt;
use super::rtl::refused::Layout;
use super::rtl::rtbt::{image_for, DUMMY, SNIPPETS};

#[test]
fn only_the_subsections_for_this_rom_are_taken() {
    let f = rtbt(&[(SNIPPETS, &[(1, 15, 0, &[0xa1; 3]), (2, 15, 0, &[0xb2; 5])])], 20);
    assert_eq!(image_for(&f, 0, 0), Ok(vec![0xa1; 3]), "eco is the ROM version plus one");
    assert_eq!(image_for(&f, 1, 0), Ok(vec![0xb2; 5]));
    assert_eq!(image_for(&f, 2, 0), Err(Layout::NoPatch));
    assert_eq!(image_for(&f, 0xff, 0), Err(Layout::NoPatch), "no eco follows FF");
}

#[test]
fn priority_orders_the_image_and_equal_priorities_run_later_first() {
    let (x, y, z, w) = (&[1u8][..], &[2u8, 2][..], &[3u8][..], &[4u8, 4, 4][..]);
    let f = rtbt(
        &[
            (SNIPPETS, &[(2, 15, 0, x), (2, 3, 0, y), (1, 0, 0, &[9]), (2, 15, 0, z)]),
            (DUMMY, &[(2, 1, 0, w)]),
        ],
        20,
    );
    assert_eq!(image_for(&f, 1, 0), Ok([w, y, z, x].concat()));
}

#[test]
fn sections_realtek_has_not_defined_are_passed_over() {
    // As linux-firmware's files end: a snippets section, then opcode 8
    // with nothing in it.
    let mut f = rtbt(&[(SNIPPETS, &[(2, 15, 0, &[7; 40])]), (8, &[])], 20);
    assert_eq!(image_for(&f, 1, 0), Ok(vec![7; 40]));
    f = rtbt(&[(4, &[(2, 0, 0, &[6; 4])]), (SNIPPETS, &[(2, 15, 0, &[7; 2])])], 20);
    assert_eq!(image_for(&f, 1, 0), Ok(vec![7; 2]), "an unknown section's subsections");
}

#[test]
fn a_file_with_nothing_to_send_is_not_a_patch() {
    let f = rtbt(&[(SNIPPETS, &[(2, 15, 0, &[])])], 20);
    assert_eq!(image_for(&f, 1, 0), Err(Layout::NoPatch), "only an empty subsection");
    assert_eq!(image_for(&rtbt(&[], 20), 1, 0), Err(Layout::NoPatch));
}
