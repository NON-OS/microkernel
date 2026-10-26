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
//! A window's frame is legible in every theme the setting offers and in
//! high contrast: its title focused or not at 4.5:1 on the bar, and each
//! control's glyph, the hovered glyphs and the focus rule at 3:1, measured
//! by WCAG 2.2's own formula here rather than the toolkit's. One process
//! of its own, because the theme store is the process's.

use nonos_toolkit::decorations::{
    control_wash, focus_rule, frame_bg, glyph_hover, glyph_rest, title_ink, CLOSE_HOVER,
    ON_CLOSE_HOVER,
};
use nonos_toolkit::theme::store::replace;
use nonos_toolkit::theme::{theme_of, SCHEMES};

#[path = "common/wcag.rs"]
mod wcag;
use wcag::{over, ratio};

#[test]
fn the_frame_is_legible_in_every_theme_and_in_high_contrast() {
    for index in 0..SCHEMES.len() as u8 {
        for high in [false, true] {
            replace(theme_of(index, high));
            let (bar, at) = (frame_bg(), format!("theme {index}, high contrast {high}"));
            let washed = over(bar, control_wash());
            let checks = [
                ("title", title_ink(true), bar, 4.5),
                ("unfocused title", title_ink(false), bar, 4.5),
                ("glyph at rest", glyph_rest(), bar, 3.0),
                ("hovered glyph", glyph_hover(), washed, 3.0),
                ("close's glyph on its red", ON_CLOSE_HOVER, CLOSE_HOVER, 3.0),
                ("focus rule", focus_rule(), bar, 3.0),
            ];
            for (what, fg, bg, floor) in checks {
                let r = ratio(fg, bg);
                assert!(r >= floor, "{at}: {what} {fg:08X} on {bg:08X} is {r:.2}:1");
            }
            let (lit, dim) = (ratio(title_ink(true), bar), ratio(title_ink(false), bar));
            assert!(lit > dim, "{at}: the unfocused title steps back ({lit:.2} to {dim:.2})");
            let rest = glyph_rest();
            assert!(rest != focus_rule() && rest != CLOSE_HOVER, "{at}: no colour at rest");
        }
    }
}
