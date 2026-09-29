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

//! Which encoding a text gets: BOM, then Content-Type charset, then <meta>
//! or @charset, then a guess; and parse handing the page over in UTF-8.

use crate::browser::http::response::charset::{decode, Doc};
use crate::browser::http::response::parse;
use crate::vectors::response;

fn html(b: &[u8], cs: Option<&str>) -> String {
    String::from_utf8(decode(b.to_vec(), cs.map(str::as_bytes), Doc::Html)).unwrap()
}

#[test]
fn a_byte_order_mark_wins_and_is_removed() {
    assert_eq!(html(b"\xef\xbb\xbfcaf\xc3\xa9", Some("windows-1252")), "caf\u{e9}");
    assert_eq!(html(b"\xff\xfec\0\xe9\0", None), "c\u{e9}");
    assert_eq!(html(b"\xfe\xff\0c\0\xe9", None), "c\u{e9}");
}

#[test]
fn undeclared_bytes_are_utf8_unless_mostly_not() {
    let mut page = "\u{e9}t\u{e9} ".repeat(100).into_bytes();
    page.push(0xa0);
    assert_eq!(html(&page, None).matches('\u{FFFD}').count(), 1, "one stray byte");
    assert_eq!(html(b"caf\xe9 cr\xe8me", None), "caf\u{e9} cr\u{e8}me", "windows-1252");
    let css = decode(b"@charset \"koi8-r\";\xf0".to_vec(), None, Doc::Css);
    assert!(String::from_utf8(css).unwrap().ends_with('\u{41f}'));
}

#[test]
fn parse_transcodes_text_and_leaves_valid_utf8_alone() {
    let raw = response(
        &["HTTP/1.1 200 OK", "Content-Type: text/html; charset=\"Shift_JIS\""],
        b"\x82\xa0",
    );
    assert_eq!(parse(&raw).unwrap().body, "\u{3042}".as_bytes());
    let img = response(&["HTTP/1.1 200 OK", "Content-Type: image/gif"], b"GIF89a\xe9");
    assert_eq!(parse(&img).unwrap().body, b"GIF89a\xe9");
    let body = b"plain \xc3\xa9".to_vec();
    let ptr = body.as_ptr();
    let same = decode(body, None, Doc::Plain);
    assert_eq!(same.as_ptr(), ptr, "no copy");
}
