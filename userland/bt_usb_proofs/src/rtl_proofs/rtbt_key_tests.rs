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

//! Security sections: a chip whose key id is 0 takes none of them; a chip
//! with a key takes only the subsections signed for its key, beside the
//! snippets every chip takes.

use super::fixture_rtbt::rtbt;
use super::rtl::refused::Layout;
use super::rtl::rtbt::{image_for, SECURITY, SNIPPETS};

fn file() -> Vec<u8> {
    let signed = [(2, 1, 1, &[0x51; 3][..]), (2, 1, 2, &[0x52; 3][..]), (1, 1, 2, &[0x50; 3][..])];
    rtbt(&[(SNIPPETS, &[(2, 15, 0, &[0xaa; 4])]), (SECURITY, &signed)], 20)
}

#[test]
fn a_chip_without_a_key_takes_no_security_section() {
    assert_eq!(image_for(&file(), 1, 0), Ok(vec![0xaa; 4]));
    let only = rtbt(&[(SECURITY, &[(2, 1, 0, &[0x50; 3])])], 20);
    assert_eq!(image_for(&only, 1, 0), Err(Layout::NoPatch), "not even one signed for key 0");
}

#[test]
fn a_keyed_chip_takes_only_what_is_signed_for_its_key() {
    let ahead = |k: u8| [vec![k; 3], vec![0xaa; 4]].concat();
    assert_eq!(image_for(&file(), 1, 2), Ok(ahead(0x52)), "priority 1 runs before 15");
    assert_eq!(image_for(&file(), 1, 1), Ok(ahead(0x51)));
    assert_eq!(image_for(&file(), 1, 3), Ok(vec![0xaa; 4]), "no subsection for key 3");
    assert_eq!(image_for(&file(), 0, 2), Ok(vec![0x50; 3]), "ROM 0 has only its signed one");
}

#[test]
fn snippets_are_taken_whatever_their_key_octet() {
    let f = rtbt(&[(SNIPPETS, &[(2, 15, 9, &[0xaa; 4])])], 20);
    assert_eq!(image_for(&f, 1, 0), Ok(vec![0xaa; 4]));
    assert_eq!(image_for(&f, 1, 2), Ok(vec![0xaa; 4]));
}
