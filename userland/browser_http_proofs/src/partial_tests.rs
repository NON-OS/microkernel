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

//! A page cut short renders what arrived; images and scripts stay whole or
//! nothing; a decompression bomb is refused while a big page is cut.

use crate::browser::http::response::{parse, ContentKind};
use crate::vectors::{read, response, zeros_deflate, PAGE};

fn cut(ct: &str, body: &[u8], keep: usize) -> Vec<u8> {
    let len = format!("Content-Length: {}", body.len());
    response(&["HTTP/1.1 200 OK", &format!("Content-Type: {ct}"), &len], &body[..keep])
}

#[test]
fn a_page_cut_short_keeps_what_arrived() {
    let r = parse(&cut("text/html", PAGE, 20)).unwrap();
    assert_eq!((r.content_kind == ContentKind::Html, &r.body[..]), (true, &PAGE[..20]));
    let css = parse(&cut("text/css", b"a{color:red}b{x:y}", 12)).unwrap();
    assert_eq!(css.body, b"a{color:red}");
}

#[test]
fn images_and_scripts_are_whole_or_nothing() {
    let png = b"\x89PNG\r\n\x1a\n\0\0\0\rIHDR";
    assert!(parse(&cut("image/png", png, 10)).is_none());
    assert!(parse(&cut("text/javascript", b"let a = 1; let b = 2;", 11)).is_none());
    /* No Content-Type: binary bytes are not a page. */
    let bare = response(&["HTTP/1.1 200 OK", "Content-Length: 16"], &png[..10]);
    assert!(parse(&bare).is_none());
    assert_eq!(parse(&cut("image/png", png, png.len())).unwrap().body, png);
}

#[test]
fn a_big_page_is_cut_at_the_inflate_cap() {
    let r = parse(&read("cases/ce_gzip_5MB_page.raw")).unwrap();
    assert_eq!((r.status, r.body.len()), (200, nonos_inflate::MAX_OUT));
}

#[test]
fn a_decompression_bomb_is_refused_and_a_big_page_is_cut() {
    /* Zeros at 258 bytes per 13 bits, gzip header, no trailer. 8,200
    matches (2.1 MB) come back whole; 40,000 (10.3 MB) are cut at the
    cap, having read over a sixteenth of the input by then; 300,000
    (77 MB) reach the cap after a fiftieth of the input: a bomb. */
    let head = ["HTTP/1.1 200 OK", "Content-Type: text/html", "Content-Encoding: gzip"];
    let gz = |m: usize| {
        let mut g = vec![0x1f, 0x8b, 8, 0, 0, 0, 0, 0, 0, 3];
        g.extend(zeros_deflate(m));
        response(&head, &g)
    };
    assert_eq!(parse(&gz(8_200)).unwrap().body.len(), 1 + 258 * 8_200);
    assert_eq!(parse(&gz(40_000)).unwrap().body.len(), nonos_inflate::MAX_OUT);
    assert!(parse(&gz(300_000)).is_none());
}
