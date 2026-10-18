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

//! Word options.

use crate::flags::parse::parse;
use crate::flags::spec::Spec;

#[test]
fn word_options_take_the_next_argument_whole() {
    let spec = Spec::new(b"find", b"").words(&[b"name", b"type"]);
    let args: [&[u8]; 5] = [b"/etc", b"-name", b"*.cfg", b"-type", b"f"];
    let p = parse(&spec, &args).unwrap();
    assert_eq!(p.word(b"name"), Some(&b"*.cfg"[..]));
    assert_eq!(p.word(b"type"), Some(&b"f"[..]));
    assert_eq!(p.operands, [b"/etc"]);
}

#[test]
fn a_word_option_is_not_treated_as_a_cluster() {
    let spec = Spec::new(b"find", b"");
    let args: [&[u8]; 1] = [b"-name"];
    assert!(parse(&spec, &args).is_err());
}
