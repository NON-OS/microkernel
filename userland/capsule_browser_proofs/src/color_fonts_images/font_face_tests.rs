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

//! @font-face selection: which source loads (WOFF2 now among them) and
//! which declared weight fills the regular and the bold slot, by CSS
//! font matching, with weight ranges parsed.

use crate::browser::fonts::{collect_font_faces, family_key};
use crate::browser::layout::boxmodel::Content;
use crate::probe::Page;

const BOLD: u32 = 1 << 31;

pub(super) fn faces(css: &str) -> Vec<(u32, String)> {
    collect_font_faces(css)
}

fn face(family: &str, weight: &str, url: &str) -> String {
    format!(
        "@font-face{{font-family:{family};src:url({url}) format('woff2');font-weight:{weight}}}"
    )
}

#[test]
fn weight_500_draws_regular_and_600_draws_bold() {
    let p = Page::at(
        "<p style='font-weight:500'>medium</p><p style='font-weight:600'>semi</p>",
        (800, 600),
    );
    let bold = |w: &str| matches!(p.word(w).content, Content::Text { bold: true, .. });
    assert!(!bold("medium"), "500 is not a bold weight");
    assert!(bold("semi"), "600 and up draw bold");
}

#[test]
fn the_400_cut_is_regular_and_a_500_cut_only_stands_in() {
    let k = family_key("A");
    let all = [face("A", "400", "a4"), face("A", "500", "a5"), face("A", "700", "a7")].concat();
    assert_eq!(faces(&all), [(k, "a4".into()), (k | BOLD, "a7".into())]);
    let k = family_key("B");
    assert_eq!(faces(&face("B", "500", "b5")), [(k, "b5".into())], "500 alone serves regular text");
    let k = family_key("C");
    let light_semibold = [face("C", "300", "c3"), face("C", "600", "c6")].concat();
    assert_eq!(faces(&light_semibold), [(k, "c3".into()), (k | BOLD, "c6".into())]);
}

#[test]
fn weight_ranges_parse_and_a_variable_face_stays_regular() {
    let k = family_key("V");
    assert_eq!(
        faces(&face("V", "100 900", "v")),
        [(k, "v".into())],
        "one variable face, no wght axis"
    );
    let k = family_key("W");
    assert_eq!(faces(&face("W", "900 600", "w")), [(k, "w".into()), (k | BOLD, "w".into())]);
    let k = family_key("X");
    let both = [face("X", "300 500", "x-roman"), face("X", "600 800", "x-bold")].concat();
    assert_eq!(faces(&both), [(k, "x-roman".into()), (k | BOLD, "x-bold".into())]);
}
