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

//! Unknown flags, missing values and bad numbers.

use crate::flags::num::parse_usize;
use crate::flags::parse::parse;
use crate::support::{head_spec, rm_spec};

#[test]
fn unknown_flag_is_an_error_not_an_operand() {
    let args: [&[u8]; 1] = [b"-z"];
    let err = parse(&rm_spec(), &args).unwrap_err();
    assert_eq!(err, b"rm: unknown flag -z".to_vec());
}

#[test]
fn unknown_letter_inside_a_cluster_is_an_error() {
    let args: [&[u8]; 1] = [b"-rz"];
    let err = parse(&rm_spec(), &args).unwrap_err();
    assert_eq!(err, b"rm: unknown flag -z".to_vec());
}

#[test]
fn a_value_flag_with_nothing_after_it_is_an_error() {
    let args: [&[u8]; 1] = [b"-n"];
    let err = parse(&head_spec(), &args).unwrap_err();
    assert_eq!(err, b"head: missing value for -n".to_vec());
}

#[test]
fn parse_usize_rejects_anything_but_digits() {
    assert_eq!(parse_usize(b"120"), Some(120));
    assert_eq!(parse_usize(b""), None);
    assert_eq!(parse_usize(b"1x"), None);
}
