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

//! Shrink-to-fit measures percentage-width atoms as auto: while the box
//! is being sized its width is unknown, so a percentage of it cannot be
//! resolved and the atom offers its content width (CSS Sizing 3, cyclic
//! percentages). The percentage then resolves against the final width.

use crate::probe::Page;

const VP: (u32, u32) = (1336, 800);

fn float_of(atom: &str) -> Page {
    let html = format!(
        "<body style=margin:0><div id=f style=float:left>\
         <span id=s style='display:inline-block;{atom}'>abcdef</span></div>"
    );
    Page::at(&html, VP)
}

/* A float holding a 50% inline-block is as wide as one holding the same
 * inline-block at auto width, not the whole line; the atom is then half
 * of that. */
#[test]
fn a_percentage_atom_measures_as_auto_in_a_float() {
    let auto_w = float_of("").rect("f")[2];
    let p = float_of("width:50%");
    assert!(auto_w > 0 && auto_w < 200, "auto measures the text: {auto_w}");
    assert_eq!(p.rect("f")[2], auto_w, "the float shrinks to the content");
    assert_eq!(p.rect("s")[2], auto_w / 2, "then 50% resolves against it");
}

/* A percentage margin on the atom counts as zero while measuring. */
#[test]
fn a_percentage_margin_adds_nothing_while_measuring() {
    let auto_w = float_of("").rect("f")[2];
    assert_eq!(float_of("margin-left:10%").rect("f")[2], auto_w);
}

/* An absolutely positioned box with an auto width shrinks to fit the same
 * way. */
#[test]
fn an_abspos_box_shrinks_around_a_percentage_atom() {
    let html = "<body style=margin:0><div id=a style='position:absolute'>\
                <span style='display:inline-block;width:50%'>abcdef</span></div>";
    let w = Page::at(html, VP).rect("a")[2];
    assert_eq!(w, float_of("").rect("f")[2]);
}
