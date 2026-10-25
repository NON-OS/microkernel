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

//! What the base block alone answers, and what refuses an EDID outright.

use crate::build::{base, seal};
use crate::edid::Edid;
use crate::error::Error;

#[test]
fn a_well_formed_base_block_parses() {
    let b = base();
    let e = Edid::parse(&b).unwrap();
    assert_eq!(&e.manufacturer(), b"LEN");
    assert_eq!(e.product(), 0x40A1);
    assert_eq!(e.serial(), 0x0102_0304);
    assert_eq!(e.version(), (1, 4));
    assert!(e.digital());
    assert!(e.at_least_1_3());
    assert_eq!(e.extension_count(), 0);
    assert_eq!(e.size_mm(), (520, 290), "centimetres stand in without a timing");
}

#[test]
fn fewer_than_128_bytes_is_short() {
    let b = base();
    for n in 0..128 {
        assert_eq!(Edid::parse(&b[..n]).unwrap_err(), Error::Short, "len {n}");
    }
}

#[test]
fn a_wrong_header_or_sum_refuses_the_panel() {
    let mut b = base();
    b[3] = 0;
    seal(&mut b);
    assert_eq!(Edid::parse(&b).unwrap_err(), Error::Header);
    let mut b = base();
    b[100] ^= 1;
    assert_eq!(Edid::parse(&b).unwrap_err(), Error::Checksum);
}

#[test]
fn an_aspect_ratio_in_the_size_bytes_is_no_size() {
    // Section 3.6.2: one of the two zero means the other is a ratio.
    let mut b = base();
    b[22] = 0;
    seal(&mut b);
    assert_eq!(Edid::parse(&b).unwrap().size_mm(), (0, 0));
    assert_eq!(Edid::parse(&b).unwrap().physical_mm(), 0);
}

#[test]
fn a_garbled_maker_code_prints_as_question_marks() {
    let mut b = base();
    b[8] = 0x7F;
    b[9] = 0xFF;
    seal(&mut b);
    assert_eq!(&Edid::parse(&b).unwrap().manufacturer(), b"???");
}
