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

/* font-weight: the number rides in the font key, a variable face is drawn
 * at it on its weight axis (heavier is wider), and bold is 600 and up. */
use super::woff2_tests::font;
use crate::browser::fonts::{family_key, ingest_font, measure_text, weight_of, weighted};
use crate::probe::Page;

#[test]
fn a_variable_face_measures_wider_as_it_gets_heavier() {
    let key = family_key("weight test geist");
    assert!(ingest_font(key, font("GeistVariable.ttf")));
    let at =
        |w: u16| measure_text(weighted(key, w), false, w >= 600, "operating proves", 100.0, 0.0);
    let (w400, w500, w700) = (at(400), at(500), at(700));
    assert!(w400 < w500 && w500 < w700, "{w400} {w500} {w700}");
    assert_eq!(w400, measure_text(key, false, false, "operating proves", 100.0, 0.0));
}

#[test]
fn weight_is_kept_through_family_and_stepped_by_bolder() {
    let html = "<style>@font-face{font-family:G;src:url(g.woff2)}</style>\
                <p id=a style=\"font-weight:500;font-family:G\">a</p>\
                <p id=b style=\"font-weight:bolder\">b</p>\
                <b id=c style=\"font-weight:lighter\">c</b>";
    let p = Page::at(html, (400, 300));
    let key = |w: &str| match p.word(w).content {
        crate::browser::layout::boxmodel::Content::Text { font, bold, .. } => {
            (weight_of(font), bold)
        }
        _ => unreachable!(),
    };
    assert_eq!(key("a"), (500, false), "a later font-family keeps the weight");
    assert_eq!(key("b"), (700, true), "bolder from 400");
    assert_eq!(key("c"), (100, false), "lighter from bold 700");
}
