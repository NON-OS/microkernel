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

//! SGR codes that set one attribute or one of the sixteen named colours.

use crate::cell::{attr, Pen};
use crate::color::Color;

fn set_underline(pen: &mut Pen, style: u16) {
    pen.attr = (pen.attr & !attr::UNDERLINE_MASK) | (style.min(5) << attr::UNDERLINE_SHIFT);
}

/// Apply group `g`, a code with any sub-parameters. Codes this terminal
/// does not draw are read and ignored.
pub(super) fn plain(pen: &mut Pen, g: &[u16]) {
    let code = g[0];
    match code {
        0 => *pen = Pen { link: pen.link, ..Pen::default() },
        1 => pen.attr |= attr::BOLD,
        2 => pen.attr |= attr::DIM,
        3 => pen.attr |= attr::ITALIC,
        4 => set_underline(pen, g.get(1).copied().unwrap_or(1)),
        5 | 6 => pen.attr |= attr::BLINK,
        7 => pen.attr |= attr::INVERSE,
        8 => pen.attr |= attr::HIDDEN,
        9 => pen.attr |= attr::STRIKE,
        21 => set_underline(pen, 2),
        22 => pen.attr &= !(attr::BOLD | attr::DIM),
        23 => pen.attr &= !attr::ITALIC,
        24 => set_underline(pen, 0),
        25 => pen.attr &= !attr::BLINK,
        27 => pen.attr &= !attr::INVERSE,
        28 => pen.attr &= !attr::HIDDEN,
        29 => pen.attr &= !attr::STRIKE,
        30..=37 => pen.fg = Color::Indexed((code - 30) as u8),
        39 => pen.fg = Color::Default,
        40..=47 => pen.bg = Color::Indexed((code - 40) as u8),
        49 => pen.bg = Color::Default,
        53 => pen.attr |= attr::OVERLINE,
        55 => pen.attr &= !attr::OVERLINE,
        90..=97 => pen.fg = Color::Indexed((code - 90 + 8) as u8),
        100..=107 => pen.bg = Color::Indexed((code - 100 + 8) as u8),
        _ => {}
    }
}
