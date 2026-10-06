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

//! SGR colours and attributes as cells record them.

#[path = "support/term.rs"]
mod support;
use nonos_vt::cell::attr;
use nonos_vt::color::Color;
use support::term;

#[test]
fn sgr_colours_in_both_notations() {
    let t = term(4, 1, "\x1b[38;2;1;2;3ma\x1b[38:2::4:5:6mb\x1b[48;5;200mc\x1b[1;4:3md");
    let l = t.visible_line(0);
    assert_eq!(l.cell(0).fg, Color::Rgb(1, 2, 3));
    assert_eq!(l.cell(1).fg, Color::Rgb(4, 5, 6));
    assert_eq!(l.cell(2).bg, Color::Indexed(200));
    assert_ne!(l.cell(3).attr & attr::BOLD, 0);
    assert_eq!(l.cell(3).underline(), nonos_vt::Underline::Curly);
}

#[test]
fn a_malformed_colour_does_not_eat_what_follows() {
    let t = term(4, 1, "\x1b[38;2;300;1;1;1ma");
    assert_ne!(t.visible_line(0).cell(0).attr & attr::BOLD, 0);
}
