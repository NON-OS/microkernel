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

//! The tokenizer on its own (13.2.5): tag and attribute states, one token
//! per call, and the text states the tree builder switches it into.

use crate::browser::html::tokenizer::TextMode;
use crate::tokens::tokens;

fn data(src: &str) -> String {
    tokens(src, TextMode::Data, "").join(" ")
}

#[test]
fn names_are_lowercased_and_the_first_duplicate_wins() {
    assert_eq!(data("<DIV ID=a id=b Class=\"x y\">"), "<div id=a class=x y");
    assert_eq!(data("<br/><div/ x><p/>"), "<br/ <div x= <p/");
    assert_eq!(data("</P x=1>"), "/p");
    assert_eq!(data("<a\0b c\0=d\0>"), "<a\u{FFFD}b c\u{FFFD}=d\u{FFFD}");
}

#[test]
fn a_less_than_sign_that_opens_nothing_is_text() {
    assert_eq!(data("a < b <3 </> c"), "'a < b <3 ' ' c'");
    assert_eq!(data("x</"), "'x' '</'");
    assert_eq!(data("<?php x ?>y</ z>w"), "# 'y' # 'w'");
    assert_eq!(data("<a"), "");
}

#[test]
fn comments_end_where_the_specification_says() {
    assert_eq!(data("<!-->a<!--->b<!---->c"), "# 'a' # 'b' # 'c'");
    assert_eq!(data("<!-- x -- y -->a<!--x--!>b"), "# 'a' # 'b'");
    assert_eq!(data("<!--x"), "#");
    assert_eq!(data("<!doctype HTML>x<!DOCTYPE>"), "!html 'x' !");
}
