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

//! A second-layout file that does not hold together is refused whole: no
//! section or subsection is read past what contains it.

use super::fixture_rtbt::rtbt;
use super::rtl::refused::Layout;
use super::rtl::rtbt::{image_for, HEADER, SNIPPETS};

fn good() -> Vec<u8> {
    rtbt(&[(SNIPPETS, &[(2, 15, 0, &[7; 16])])], 20)
}

#[test]
fn the_signature_and_header_are_checked() {
    let mut f = good();
    f[0] = b'r';
    assert_eq!(image_for(&f, 1, 0), Err(Layout::Signature));
    assert_eq!(image_for(&good()[..HEADER - 1], 1, 0), Err(Layout::Signature));
    assert_eq!(image_for(b"Realtech\x01\x02\x03\x04\x01\x00", 1, 0), Err(Layout::Signature));
}

#[test]
fn a_section_past_the_end_is_refused() {
    let mut f = good();
    f[16] = 2;
    f.truncate(f.len() - 8);
    assert_eq!(image_for(&f, 1, 0), Err(Layout::Table), "a second section the file lacks");
    let mut f = good();
    f[26] = 1;
    assert_eq!(image_for(&f, 1, 0), Err(Layout::Bounds), "a section longer than the file");
    let f = good();
    assert_eq!(image_for(&f[..f.len() - 30], 1, 0), Err(Layout::Bounds), "a cut file");
}

#[test]
fn a_subsection_past_its_section_is_refused() {
    let mut f = good();
    // The subsection's length, eight octets into the section's body.
    f[HEADER + 8 + 8] = 17;
    assert_eq!(image_for(&f, 1, 0), Err(Layout::Bounds), "one octet past its section");
    let mut f = good();
    f[HEADER + 8] = 2;
    assert_eq!(image_for(&f, 1, 0), Err(Layout::Bounds), "a second subsection it lacks");
    let mut f = rtbt(&[(SNIPPETS, &[])], 20);
    f[HEADER + 4] = 3;
    f.truncate(HEADER + 8 + 3);
    assert_eq!(image_for(&f, 1, 0), Err(Layout::Bounds), "a body without its count");
}

#[test]
fn a_bad_subsection_for_another_rom_still_refuses_the_file() {
    let mut f = rtbt(&[(SNIPPETS, &[(1, 15, 0, &[5; 4]), (2, 15, 0, &[7; 4])])], 20);
    // The second subsection, for ROM 1, made to run past its section.
    f[HEADER + 8 + 4 + 8 + 4 + 4] = 9;
    assert_eq!(image_for(&f, 0, 0), Err(Layout::Bounds));
}
