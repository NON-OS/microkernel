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

//! The table insertion modes (13.2.6.4.9 to 13.2.6.4.15): implied sections
//! and rows, foster parenting, and what a table ignores.

use crate::shape::body;

fn each(cases: &[(&str, &str)]) {
    for (html, want) in cases {
        assert_eq!(body(html), *want, "{html}");
    }
}

#[test]
fn sections_and_rows_are_implied() {
    each(&[
        ("<table><td>a", "<table><tbody><tr><td>a</td></tr></tbody></table>"),
        ("<table><tr><th>a<td>b", "<table><tbody><tr><th>a</th><td>b</td></tr></tbody></table>"),
        ("<table><col><tr>", "<table><colgroup><col></colgroup><tbody><tr></tr></tbody></table>"),
        (
            "<table><caption>c<tr><td>x</table>",
            "<table><caption>c</caption><tbody><tr><td>x</td></tr></tbody></table>",
        ),
        ("<table><thead><tr><tbody>", "<table><thead><tr></tr></thead><tbody></tbody></table>"),
    ]);
}

#[test]
fn stray_content_is_fostered_before_the_table() {
    each(&[
        ("<table>x<tr><td>y</table>", "x<table><tbody><tr><td>y</td></tr></tbody></table>"),
        ("<table><b>x</b><tr><td>y", "<b>x</b><table><tbody><tr><td>y</td></tr></tbody></table>"),
        ("<table> <tr> </tr></table>", "<table> <tbody><tr> </tr></tbody></table>"),
        ("<table><div>a</div></table>", "<div>a</div><table></table>"),
    ]);
}

#[test]
fn some_elements_stay_inside_the_table() {
    each(&[
        ("<table><input type=hidden></table>", "<table><input type=\"hidden\"></table>"),
        ("<table><form><tr></table>", "<table><form></form><tbody><tr></tr></tbody></table>"),
        ("<table></td></tr></tbody>x</table>", "x<table></table>"),
        (
            "<table><tr><td><table><tr><td>i</table>o</table>",
            concat!(
                "<table><tbody><tr><td>",
                "<table><tbody><tr><td>i</td></tr></tbody></table>",
                "o</td></tr></tbody></table>"
            ),
        ),
    ]);
}
