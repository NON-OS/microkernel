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

//! The HTML prescan for a declared encoding: <meta> forms, comments, the
//! UTF-16 and x-user-defined special cases, XML declarations, 8 KiB reach.

use crate::browser::http::response::charset::{decode, Doc};

fn html(b: &[u8], cs: Option<&str>) -> String {
    String::from_utf8(decode(b.to_vec(), cs.map(str::as_bytes), Doc::Html)).unwrap()
}

#[test]
fn the_header_then_a_meta_then_a_guess_decide() {
    let meta = b"<!doctype html><meta charset=\"windows-1250\"><p>Probl\xe9my \x9a";
    assert!(html(meta, None).ends_with("Probl\u{e9}my \u{161}"), "meta charset");
    assert!(html(meta, Some("iso-8859-2")).ends_with("\u{9a}"), "header over meta");
    let pragma = b"<meta http-equiv=Content-Type content='text/html; charset=koi8-r'>\xf0\xd2";
    assert!(html(pragma, None).ends_with("\u{41f}\u{440}"));
    let no_pragma = b"<meta content='text/html; charset=koi8-r'>\xf0\xd2";
    assert!(html(no_pragma, None).ends_with("\u{f0}\u{d2}"), "content alone declares nothing");
    let hidden = b"<!-- <meta charset=koi8-r> --><p>\xe9";
    assert!(html(hidden, None).ends_with("\u{e9}"), "a meta in a comment is skipped");
    assert!(
        html(b"<meta charset=utf-16le>\xc3\xa9", None).ends_with("\u{e9}"),
        "UTF-16 means UTF-8"
    );
    assert!(html(b"<meta charset=x-user-defined>\x80", None).ends_with("\u{20ac}"));
    assert!(
        html(b"<?xml version=\"1.0\" encoding=\"Shift_JIS\"?>\x82\xa0", None).ends_with("\u{3042}")
    );
}

#[test]
fn a_meta_past_1024_bytes_counts_only_for_non_utf8_bytes() {
    let mut late = vec![b' '; 2000];
    late.extend_from_slice(b"<meta charset=koi8-r>\xf0");
    assert!(html(&late, None).ends_with("\u{41f}"));
    assert!(html(&late[..2021], None).ends_with('>'), "valid UTF-8 needs no declaration");
}
