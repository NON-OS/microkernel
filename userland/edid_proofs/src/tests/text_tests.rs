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

//! Name and serial strings (E-EDID 1.4 sections 3.10.3.1 and 3.10.3.4).

use crate::build::{base, display, put};
use crate::edid::Edid;

#[test]
fn a_name_ends_at_its_line_feed_and_loses_its_padding() {
    let mut b = base();
    put(&mut b, 0, &display(0xFC, b"DELL U2720Q\n "));
    put(&mut b, 1, &display(0xFF, b"ABC123  \n    "));
    let e = Edid::parse(&b).unwrap();
    assert_eq!(e.name().unwrap().as_bytes(), b"DELL U2720Q");
    assert_eq!(e.serial_text().unwrap().as_bytes(), b"ABC123");
}

#[test]
fn a_name_of_all_thirteen_bytes_has_no_terminator() {
    let mut b = base();
    put(&mut b, 2, &display(0xFC, b"ABCDEFGHIJKLM"));
    assert_eq!(Edid::parse(&b).unwrap().name().unwrap().as_bytes(), b"ABCDEFGHIJKLM");
}

#[test]
fn no_name_descriptor_is_no_name() {
    assert!(Edid::parse(&base()).unwrap().name().is_none());
}
