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

//! Whose claim stops the console for good (L79): a function whose own BAR
//! holds the framebuffer, or, when no BAR anywhere holds it, the display
//! function if it is the only one. The rule over all its inputs.

use super::cover_rule::covers;

/// Every row of `covers(mine, any, display, single_display)`, its answer
/// written out by hand rather than computed.
const TABLE: [(bool, bool, bool, bool, bool); 16] = [
    (false, false, false, false, false),
    (false, false, false, true, false),
    (false, false, true, false, false),
    (false, false, true, true, true),
    (false, true, false, false, false),
    (false, true, false, true, false),
    (false, true, true, false, false),
    (false, true, true, true, false),
    (true, false, false, false, true),
    (true, false, false, true, true),
    (true, false, true, false, true),
    (true, false, true, true, true),
    (true, true, false, false, true),
    (true, true, false, true, true),
    (true, true, true, false, true),
    (true, true, true, true, true),
];

#[test]
fn covers_answers_every_row_of_its_table() {
    for (mine, any, display, single, want) in TABLE {
        let got = covers(mine, any, display, single);
        assert_eq!(got, want, "mine {mine} any {any} display {display} single {single}");
    }
}
