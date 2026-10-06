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

//! Under the list: what the selected entry does, and whether this machine
//! meets what its policy will require.

use super::entries::ENTRIES;
use super::layout::Layout;
use super::ready::{missing, policy_of};
use crate::display::ink::palette::{BAD, CYAN, TEXT, TEXT_3};
use crate::display::ink::{dot, draw_wrapped, label, metrics, Style};
use crate::display::text::Text;
use crate::security::SecurityContext;

/// Lines the selected entry's sentence may take.
pub(super) const ABOUT_LINES: u32 = 1;

pub(super) fn draw_about(l: &Layout, sel: usize, sec: &SecurityContext) {
    let u = l.u;
    let body = metrics(Style::Body);
    draw_wrapped(l.col_x, l.about_y, l.col_w, ENTRIES[sel].about, Style::Body, TEXT, 2);
    label(l.col_x, l.about_y + body.line + 2 * u, ENTRIES[sel].spec, TEXT_3);
    let Some(policy) = policy_of(ENTRIES[sel].action) else { return };
    let m = missing(sec, policy);
    let y = l.about_y + body.line * 2 + metrics(Style::Mono).line + 6 * u;
    let mut t = Text::new();
    let color = if m.len == 0 {
        t = t.push(b"READY ON THIS MACHINE");
        CYAN
    } else {
        t = t.push(b"REFUSED HERE: NO ");
        for (i, name) in m.names[..m.len].iter().enumerate() {
            t = t.push(if i == 0 { b"" } else { b", " });
            t = name.iter().fold(t, |t, &b| t.push(&[b.to_ascii_uppercase()]));
        }
        BAD
    };
    let d = dot(l.col_x, y, Style::Mono, color);
    label(l.col_x + d + 2 * u, y, t.as_bytes(), color);
}
