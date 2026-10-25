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

//! A battery's static information, built in AML and read by the
//! interpreter: _BIF and _BIX.

use crate::enc::*;
use crate::power::{parse_bif, parse_bix};
use crate::run::eval;

const UNKNOWN: u64 = 0xFFFF_FFFF;

fn bif() -> Vec<u8> {
    let ints = [1, 5000, 4500, 1, 11100, 450, 150, 10, 10].map(int);
    let strs = ["BAT-1", "0042", "LION", "NONOS"].map(string);
    package(13, &[&ints[..], &strs[..]].concat())
}

#[test]
fn bif_decodes_in_milliamp_hours() {
    let i = parse_bif(&eval(&ret(&bif()), vec![]).unwrap()).unwrap();
    assert!(i.milliamp && i.rechargeable);
    assert_eq!(
        (i.design_capacity, i.last_full, i.design_voltage),
        (Some(5000), Some(4500), Some(11100))
    );
    assert_eq!((i.warning, i.low, i.cycle_count), (Some(450), Some(150), None));
    assert_eq!(
        (&i.model[..], &i.serial[..], &i.kind[..], &i.oem[..]),
        (&b"BAT-1"[..], &b"0042"[..], &b"LION"[..], &b"NONOS"[..])
    );
}

#[test]
fn bix_carries_the_cycle_count_and_unknowns_stay_unknown() {
    let ints =
        [1, 0, 50000, UNKNOWN, 1, 11400, 5000, 1500, 321, 98000, 0, 0, 0, 0, 10, 10].map(int);
    let strs = ["M", "S", "LiP", "O"].map(string);
    let bix = package(20, &[&ints[..], &strs[..]].concat());
    let i = parse_bix(&eval(&ret(&bix), vec![]).unwrap()).unwrap();
    assert!(!i.milliamp, "milliwatt hours");
    assert_eq!((i.design_capacity, i.last_full, i.cycle_count), (Some(50000), None, Some(321)));
    assert_eq!(&i.kind[..], b"LiP");
}
