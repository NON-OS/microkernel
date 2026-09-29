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

//! Items locked to a row (a definite row, an auto column) place from the
//! row's cursor on and never before it under sparse placement (Grid 8.5
//! step 2), making implicit columns when the explicit ones are full;
//! dense placement may fill a hole earlier in the row.

use crate::probe::Page;

const VP: (u32, u32) = (1336, 800);

/* Four 100px columns; x holds the second, a (span 2) jumps it to the
 * third and fourth, leaving the first empty behind the cursor. */
fn row(flow: &str) -> Page {
    let html = format!(
        "<body style=margin:0><div style='display:grid;width:800px;\
         grid-template-columns:repeat(4,100px);grid-auto-flow:{flow}'>\
         <div id=x style='grid-row:1;grid-column:2'>x</div>\
         <div id=a style='grid-row:1;grid-column:span 2'>a</div>\
         <div id=b style='grid-row:1'>b</div></div>"
    );
    Page::at(&html, VP)
}

/* Sparse: b goes past a, into a new fifth column, not back to the first
 * as it did; dense placement may fill that hole. */
#[test]
fn a_row_locked_item_never_goes_before_the_cursor() {
    let p = row("row");
    let (a, b) = (p.rect("a"), p.rect("b"));
    assert_eq!((a[0], a[2]), (200, 200), "a spans the third and fourth");
    assert_eq!(b[1], a[1], "b stays in row 1");
    assert!(b[0] >= 400, "b is past a, in an implicit column: {b:?}");
    let p = row("row dense");
    assert_eq!(p.rect("b")[0..2], [0, p.rect("a")[1]], "dense fills the hole");
}
