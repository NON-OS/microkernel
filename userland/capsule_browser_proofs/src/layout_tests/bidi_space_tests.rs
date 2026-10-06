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

//! Reordered lines keep one space in every gap: a space between runs of
//! two directions takes the embedding direction (UAX #9 N1, N2) and stays
//! between the runs when rule L2 reverses them.

use crate::browser::layout::boxmodel::Fragment;
use crate::probe::Page;

const VP: (u32, u32) = (1336, 800);
const SHALOM: &str = "\u{5e9}\u{5dc}\u{5d5}\u{5dd}";
const OLAM: &str = "\u{5e2}\u{5d5}\u{5dc}\u{5dd}";

fn gap(a: &Fragment, b: &Fragment) -> i32 {
    b.x - (a.x + a.w)
}

/* The width of one collapsed space between two words, left to right. */
fn one_space() -> i32 {
    let p = Page::at("<body style=margin:0><p style=margin:0>go now</p>", VP);
    gap(p.word("go"), p.word("now"))
}

fn rev(s: &str) -> String {
    s.chars().rev().collect()
}

/* "go A B now" with A B Hebrew shows as go B A now: each of the three
 * gaps holds one space, none is empty and none doubled. */
#[test]
fn an_rtl_run_in_ltr_text_keeps_one_space_per_gap() {
    let html = format!("<body style=margin:0><p style=margin:0>go {SHALOM} {OLAM} now</p>");
    let p = Page::at(&html, VP);
    let (a, b) = (rev(SHALOM), rev(OLAM));
    let (go, a, b, now) = (p.word("go"), p.word(&a), p.word(&b), p.word("now"));
    let sp = one_space();
    assert!(sp > 0);
    assert_eq!([gap(go, b), gap(b, a), gap(a, now)], [sp, sp, sp]);
}

/* "A abc def B" in an rtl paragraph shows as B abc def A, right to left:
 * the ltr run keeps its inner space and one space on each side. */
#[test]
fn an_ltr_run_in_rtl_text_keeps_one_space_per_gap() {
    let html = format!(
        "<body style=margin:0><p dir=rtl style='direction:rtl;width:600px;margin:0'>\
         {SHALOM} abc def {OLAM}</p>"
    );
    let p = Page::at(&html, VP);
    let (a, b) = (rev(SHALOM), rev(OLAM));
    let (a, abc, def, b) = (p.word(&a), p.word("abc"), p.word("def"), p.word(&b));
    let sp = one_space();
    assert_eq!([gap(b, abc), gap(abc, def), gap(def, a)], [sp, sp, sp]);
    assert_eq!(a.x + a.w, 600, "the line still ends at the right edge");
}
