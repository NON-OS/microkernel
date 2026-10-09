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

//! The foot of the menu, under a full rule: the countdown as a line that
//! shortens, and the keys, all in mono capitals.

use super::entries::ENTRIES;
use super::layout::Layout;
use crate::display::gop::{get_dimensions, hline};
use crate::display::ink::palette::{BORDER, CYAN, TEXT_2, TEXT_3};
use crate::display::ink::{label, label_width, metrics, round_rect, Style};
use crate::display::text::Text;

const KEYS: &[u8] = b"\x18\x19 SELECT   1-7 JUMP   ENTER START   ESC HOLD";

/// `remaining_s` of `total_s` left; zero once the person has pressed a key.
pub(super) fn draw_footer(l: &Layout, remaining_s: u32, total_s: u32, default: usize) {
    let (w, _) = get_dimensions();
    let (u, mono) = (l.u, metrics(Style::Mono));
    let side = 8 * u;
    hline(0, l.footer_y, w, BORDER);
    let y = l.footer_y + mono.line + 2 * u;
    let t = if remaining_s == 0 {
        Text::new().push(b"TIMER HELD")
    } else {
        let name = upper(ENTRIES[default].label);
        Text::new().push(name.as_bytes()).push(b" IN ").dec(remaining_s as u64).push(b" S")
    };
    let tw = label(side, y, t.as_bytes(), if remaining_s == 0 { TEXT_3 } else { TEXT_2 });
    if remaining_s > 0 {
        let bw = (tw * remaining_s / total_s.max(1)).max(2);
        round_rect(side, y + mono.line + u, bw, 2, 1, CYAN);
    }
    label(w.saturating_sub(side + label_width(KEYS)), y, KEYS, TEXT_3);
}

fn upper(s: &[u8]) -> Text {
    s.iter().fold(Text::new(), |t, &b| t.push(&[b.to_ascii_uppercase()]))
}
