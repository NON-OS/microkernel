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
fn the_plain_strips_carry_no_escape_at_all() {
    for row in 0..2 {
        let (plain, _) = palette(row);
        assert!(!plain.contains(&0x1b));
        assert_eq!(text(&plain).chars().count(), 16);
    }
}

/// One colour escape per block, then one reset.
#[test]
fn the_styled_strips_carry_one_escape_per_block_and_a_reset() {
    for row in 0..2 {
        let (plain, styled) = palette(row);
        assert_eq!(styled.iter().filter(|b| **b == 0x1b).count(), 9);
        assert!(styled.ends_with(b"\x1b[0m"));
        assert_eq!(strip(&styled), text(&plain));
    }
}

/// The standard eight, in order, then their bright forms; a row past the
/// last is the bright one.
#[test]
fn the_strips_name_the_standard_colours_in_order() {
    let codes = |row| text(&palette(row).1);
    for (row, base) in [(0usize, 30), (1, 90)] {
        let line = codes(row);
        let at: Vec<_> = (0..8).map(|i| line.find(&format!("\x1b[{}m", base + i))).collect();
        assert!(at.iter().all(Option::is_some), "{line:?}");
        assert!(at.windows(2).all(|w| w[0] < w[1]), "{line:?}");
    }
    assert_eq!(codes(5), codes(1));
}
