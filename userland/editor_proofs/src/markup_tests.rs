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

//! Insert > Link and Insert > Image write Markdown at the caret, and the
//! shell's open-argument reply yields the path it carried, or nothing.

use crate::edit_tests::{doc, text};
use crate::open_arg_reply::reply_path;

#[test]
fn a_link_with_nothing_selected_leaves_the_caret_in_the_brackets() {
    let mut s = doc("see ");
    s.caret = 4;
    assert!(s.insert_markup(false));
    assert_eq!(text(&s), "see []()");
    assert_eq!(s.caret, 5);
    assert!(s.undo());
    assert_eq!(text(&s), "see ");
}

#[test]
fn a_selection_becomes_the_link_text_and_the_caret_waits_for_the_address() {
    let mut s = doc("read the docs today");
    s.sel_anchor = Some(9);
    s.caret = 13;
    assert!(s.insert_markup(false));
    assert_eq!(text(&s), "read the [docs]() today");
    assert_eq!(s.caret, 16, "between the parentheses");
    assert!(s.sel_range().is_none());
    assert!(s.undo(), "one step undoes the whole insert");
    assert_eq!(text(&s), "read the docs today");
}

#[test]
fn an_image_is_the_link_with_a_bang() {
    let mut s = doc("logo");
    s.sel_anchor = Some(0);
    s.caret = 4;
    assert!(s.insert_markup(true));
    assert_eq!(text(&s), "![logo]()");
    assert_eq!(s.caret, 8);
    let mut e = doc("");
    assert!(e.insert_markup(true));
    assert_eq!(text(&e), "![]()");
    assert_eq!(e.caret, 2);
}

#[test]
fn the_open_reply_carries_a_path_or_nothing() {
    let mut body = vec![0u8; 4];
    assert_eq!(reply_path(&body), None, "a bare status holds no file");
    assert_eq!(reply_path(&body[..2]), None);
    body.extend_from_slice(b"/home/nonos/notes.txt");
    assert_eq!(reply_path(&body), Some("/home/nonos/notes.txt"));
    let mut bad = vec![0u8; 4];
    bad.extend_from_slice(b"notes.txt");
    assert_eq!(reply_path(&bad), None, "only an absolute path is opened");
    bad.truncate(4);
    bad.extend_from_slice(&[b'/', 0xFF]);
    assert_eq!(reply_path(&bad), None);
}
