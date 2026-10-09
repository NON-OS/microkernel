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

//! A pasted line into the process monitor's search field.

use nonos_app_skeleton::input::text::first_line;

use crate::pm_query_paste::{paste_query, QueryPaste};

const QUERY_MAX: usize = 24;

#[test]
fn a_copied_process_name_goes_in_after_what_was_typed() {
    let mut buf = [0u8; QUERY_MAX];
    buf[..3].copy_from_slice(b"net");
    let line = first_line(b"_core\n").expect("text");
    assert_eq!(paste_query(&mut buf, 3, line.text), QueryPaste::Took { len: 8, cut: false });
    assert_eq!(&buf[..8], b"net_core");
}

#[test]
fn a_long_line_is_cut_to_the_field() {
    let mut buf = [0u8; QUERY_MAX];
    let r = paste_query(&mut buf, 0, &"x".repeat(40));
    assert_eq!(r, QueryPaste::Took { len: QUERY_MAX, cut: true });
}

#[test]
fn a_tab_is_a_space_and_controls_are_dropped() {
    let mut buf = [0u8; QUERY_MAX];
    let r = paste_query(&mut buf, 0, "a\tb\u{7}c");
    assert_eq!(r, QueryPaste::Took { len: 4, cut: false });
    assert_eq!(&buf[..4], b"a bc");
}

#[test]
fn text_no_process_name_holds_is_refused_whole() {
    let mut buf = [0u8; QUERY_MAX];
    buf[0] = b'z';
    assert_eq!(paste_query(&mut buf, 1, "\u{201c}quoted\u{201d}"), QueryPaste::Refused);
    assert_eq!(paste_query(&mut buf, 1, "caf\u{e9}"), QueryPaste::Refused);
    assert_eq!(buf[1], 0, "nothing went in");
    assert_eq!(paste_query(&mut buf, 1, ""), QueryPaste::Empty);
}
