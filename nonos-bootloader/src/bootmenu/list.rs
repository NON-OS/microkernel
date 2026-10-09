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

use super::entries::ENTRIES;
use super::layout::Layout;
use crate::display::gop::hline;
use crate::display::ink::palette::{BORDER, CYAN, TEXT, TEXT_2, TEXT_3};
use crate::display::ink::{draw, label, label_width, marker, metrics, round_rect, Style};

/// The entries, numbered as the brand numbers its sections, on thin rules.
/// The selected one is lit: its number in cyan, a cyan bar at its edge.
pub(super) fn draw_list(l: &Layout, sel: usize, default: usize) {
    let mono = metrics(Style::Mono);
    marker(l.col_x, l.list_y - mono.line - 4 * l.u, b"START", CYAN, TEXT_2);
    for i in 0..ENTRIES.len() {
        draw_row(l, i, i == sel, i == default);
    }
    hline(l.col_x, l.list_y + l.row_h * ENTRIES.len() as u32, l.col_w, BORDER);
}

fn draw_row(l: &Layout, i: usize, selected: bool, default: bool) {
    let u = l.u;
    let top = l.list_y + l.row_h * i as u32;
    hline(l.col_x, top, l.col_w, BORDER);
    let (mono, lab) = (metrics(Style::Mono), metrics(Style::Label));
    let num = [b'0', b'1' + i as u8];
    let ny = top + l.row_h.saturating_sub(mono.line) / 2;
    let ly = top + l.row_h.saturating_sub(lab.line) / 2;
    let text_x = l.col_x + 3 * u + label_width(b"00") + 3 * u;
    if selected {
        round_rect(l.col_x, top + 2 * u, (u / 2).max(2), l.row_h.saturating_sub(4 * u), 1, CYAN);
    }
    label(l.col_x + 3 * u, ny, &num, if selected { CYAN } else { TEXT_3 });
    draw(text_x, ly, ENTRIES[i].label, Style::Label, if selected { TEXT } else { TEXT_2 });
    if default {
        let tag: &[u8] = b"DEFAULT";
        let tx = (l.col_x + l.col_w).saturating_sub(label_width(tag));
        label(tx, ny, tag, if selected { CYAN } else { TEXT_3 });
    }
}
