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

//! The JSON reader and writer.

use crate::proto::json::{parse, push_quoted, Value};

#[test]
fn reads_nested_values() {
    let v = parse(r#" {"a":[1,-2.5e3,true,false,null],"b":{"c":"d"}} "#).unwrap();
    assert_eq!(v.get("a").items().len(), 5);
    assert_eq!(v.get("a").items()[0].int(), Some(1));
    assert_eq!(v.get("a").items()[1].int(), Some(-2));
    assert_eq!(v.get("a").items()[2], Value::Bool(true));
    assert_eq!(v.at(&["b", "c"]).str(), Some("d"));
    assert_eq!(v.at(&["b", "x", "y"]), &Value::Null);
}

#[test]
fn reads_escapes_and_surrogate_pairs() {
    let v = parse(r#""a\"b\\c\né😀\/""#).unwrap();
    assert_eq!(v.str(), Some("a\"b\\c\né😀/"));
}

#[test]
fn reads_utf8_as_is() {
    assert_eq!(parse("\"NØNOS\"").unwrap().str(), Some("NØNOS"));
}

#[test]
fn refuses_broken_text() {
    for bad in ["", "{", "[1,]", "{\"a\" 1}", "\"open", "tru", "{} x", "[1 2]"] {
        assert!(parse(bad).is_none(), "{bad}");
    }
}

#[test]
fn refuses_deep_nesting() {
    let deep = "[".repeat(100) + &"]".repeat(100);
    assert!(parse(&deep).is_none());
    let fine = "[".repeat(20) + &"]".repeat(20);
    assert!(parse(&fine).is_some());
}

#[test]
fn writes_what_it_reads() {
    let text = "quote \" back \\ line\nnul \u{1} tab\té";
    let mut out = String::new();
    push_quoted(&mut out, text);
    assert_eq!(out, "\"quote \\\" back \\\\ line\\nnul \\u0001 tab\\té\"");
    assert_eq!(parse(&out).unwrap().str(), Some(text));
}
