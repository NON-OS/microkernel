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

//! Where a reference is left as written: a legacy name inside an attribute
//! value that reads as a query string, and anything `push_decoded` does not
//! know.

use crate::browser::html::entity::push_decoded;
use crate::entity_tests::cref;
use crate::shape::body;

#[test]
fn an_attribute_keeps_a_legacy_name_before_equals_or_a_letter() {
    assert_eq!(cref("copy=2", true), ("&copy".into(), 4));
    assert_eq!(cref("copy=2", false), ("\u{A9}".into(), 4));
    assert_eq!(cref("copyx", true), ("&copy".into(), 4));
    assert_eq!(cref("copy;=2", true), ("\u{A9}".into(), 5));
    assert_eq!(cref("copy ", true), ("\u{A9}".into(), 4));
    let a = body("<a href=\"?x=1&copy=2&amp;y=&lt\">AT&T &notit; &amp</a>");
    assert_eq!(a, "<a href=\"?x=1&amp;copy=2&amp;y=&lt;\">AT&amp;T \u{AC}it; &amp;</a>");
}

#[test]
fn push_decoded_keeps_what_it_does_not_know_literal() {
    let decode = |name: &str| {
        let mut s = String::new();
        push_decoded(&mut s, name);
        s
    };
    assert_eq!(decode("bogus"), "&bogus;");
    assert_eq!(decode("#0"), "&#0;");
    assert_eq!(decode("amp"), "&");
    assert_eq!(decode("#x41"), "A");
    assert_eq!(decode("nbsp"), "\u{A0}");
}
