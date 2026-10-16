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

//! The plain and styled colour strips.

use crate::neofetch::palette::palette;
use crate::support::{strip, text};

/// The whole point of the (plain, styled) pair: a redirected `neofetch` must
/// write glyphs to the file, never terminal control codes.
#[test]
fn the_plain_strip_carries_no_escape_at_all() {
    let (plain, _) = palette();
    assert!(!plain.contains(&0x1b));
    assert_eq!(text(&plain).chars().count(), 16);
}

#[test]
fn the_styled_strip_carries_one_escape_per_block() {
    let (plain, styled) = palette();
    assert_eq!(styled.iter().filter(|b| **b == 0x1b).count(), 8);
    assert_eq!(strip(&styled), text(&plain));
}

/// The strip is the point of reference for every other colour the terminal
/// prints, so the codes are the standard eight and stay in order.
#[test]
fn the_styled_strip_names_the_standard_colours_in_order() {
    let (_, styled) = palette();
    let line = text(&styled);
    for code in [
        "\x1b[31m", "\x1b[32m", "\x1b[33m", "\x1b[34m", "\x1b[35m", "\x1b[36m", "\x1b[37m",
        "\x1b[90m",
    ] {
        assert!(line.contains(code), "{code:?}");
    }
    assert!(line.find("\x1b[31m") < line.find("\x1b[90m"));
}
