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

//! The console line builder: the "igc: " prefix `log igc` finds, numbers
//! without a formatter, and a cut that still ends the line.

use nonos_libc::said;

use crate::log::Line;

fn last() -> String {
    said().last().cloned().unwrap_or_default()
}

#[test]
fn text_hex_and_decimal_go_on_one_prefixed_line() {
    Line::new().text("up ").hex(0x125C, 4).text(" ").dec(0).text(" ").dec(2500).send();
    assert_eq!(last(), "igc: up 125c 0 2500\n");
    Line::new().hex(0xC2, 2).text(":").hex(0x0F, 2).dec(u32::MAX).send();
    assert_eq!(last(), "igc: c2:0f4294967295\n");
}

#[test]
fn an_overlong_line_is_cut_and_still_ends_in_a_newline() {
    let long = "x".repeat(400);
    Line::new().text(&long).send();
    let line = last();
    assert_eq!(line.len(), 128);
    assert!(line.starts_with("igc: xxx"));
    assert!(line.ends_with("x\n"));
}
