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

//! Value flags: separate, attached, numeric shorthand.

use crate::flags::parse::parse;
use crate::support::head_spec;

#[test]
fn separate_value_flag_takes_the_next_argument() {
    let args: [&[u8]; 3] = [b"-n", b"5", b"f.txt"];
    let p = parse(&head_spec(), &args).unwrap();
    assert_eq!(p.value(b'n'), Some(&b"5"[..]));
    assert_eq!(p.operands, [b"f.txt"]);
}

#[test]
fn attached_value_flag_takes_the_cluster_remainder() {
    let args: [&[u8]; 1] = [b"-n5"];
    let p = parse(&head_spec(), &args).unwrap();
    assert_eq!(p.value(b'n'), Some(&b"5"[..]));
    assert!(p.operands.is_empty());
}

#[test]
fn numeric_shorthand_is_the_same_as_the_count_flag() {
    let args: [&[u8]; 2] = [b"-5", b"f.txt"];
    let p = parse(&head_spec(), &args).unwrap();
    assert_eq!(p.value(b'n'), Some(&b"5"[..]));
    assert_eq!(p.operands, [b"f.txt"]);
}

#[test]
fn a_value_flag_may_trail_a_boolean_in_one_cluster() {
    let args: [&[u8]; 2] = [b"-qn", b"3"];
    let p = parse(&head_spec(), &args).unwrap();
    assert!(p.has(b'q'));
    assert_eq!(p.value(b'n'), Some(&b"3"[..]));
}
