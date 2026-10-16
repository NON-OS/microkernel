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

//! Boolean flags, clusters, operands and `--`.

use crate::flags::parse::parse;
use crate::support::rm_spec;

#[test]
fn no_flags_leaves_every_arg_an_operand() {
    let args: [&[u8]; 2] = [b"a.txt", b"b.txt"];
    let p = parse(&rm_spec(), &args).unwrap();
    assert_eq!(p.operands, [b"a.txt", b"b.txt"]);
    assert!(!p.has(b'r'));
}

#[test]
fn clustered_flags_split_into_each_letter() {
    let args: [&[u8]; 2] = [b"-rf", b"x"];
    let p = parse(&rm_spec(), &args).unwrap();
    assert!(p.has(b'r') && p.has(b'f'));
    assert_eq!(p.operands, [b"x"]);
}

#[test]
fn flags_are_accepted_after_operands() {
    let args: [&[u8]; 3] = [b"x", b"-r", b"-f"];
    let p = parse(&rm_spec(), &args).unwrap();
    assert!(p.has(b'r') && p.has(b'f'));
    assert_eq!(p.operands, [b"x"]);
}

#[test]
fn double_dash_ends_flag_parsing() {
    let args: [&[u8]; 3] = [b"-r", b"--", b"-f"];
    let p = parse(&rm_spec(), &args).unwrap();
    assert!(p.has(b'r'));
    assert!(!p.has(b'f'));
    assert_eq!(p.operands, [b"-f"]);
}

#[test]
fn a_bare_dash_is_an_operand() {
    let args: [&[u8]; 1] = [b"-"];
    let p = parse(&rm_spec(), &args).unwrap();
    assert_eq!(p.operands, [b"-"]);
}
