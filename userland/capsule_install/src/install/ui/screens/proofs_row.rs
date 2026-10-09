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

//! One proof as a numbered row on a thin rule, as the bootloader shows its
//! own: the number and the name, the state word in mono at the right, what
//! the word rests on under the name, and the short hex of what was proven.
//! The word carries the verdict in letters, so colour is never the only sign.

use alloc::format;

use nonos_app_skeleton::PaintBuffer;
use nonos_brand::{label, label_w};

use super::proofs_tint::tint;
use crate::install::proofs::Row;
use crate::install::ui::metrics::Metrics;
use crate::install::ui::text::{fit, top_of};
use crate::install::ui::{text, theme};

/// The number's column, in units.
const NUM_W: u32 = 5;

/// Paints proof number `n` at `y` and returns the y of the next.
pub fn row(fb: &mut PaintBuffer, m: &Metrics, x: u32, y: u32, w: u32, n: usize, p: &Row) -> u32 {
    fb.fill_rect(x, y, w, 1, theme::RULE);
    let (line, lpx) = (m.line_h, m.label_px);
    let top = y + m.unit;
    label(fb, x, top_of(top, line, lpx), &format!("{:02}", n), theme::MUTED, lpx);
    let word = p.mark.word();
    let ww = label_w(word, lpx);
    label(fb, (x + w).saturating_sub(ww), top_of(top, line, lpx), word, tint(p.mark), lpx);
    let num_w = NUM_W * m.unit;
    let (tx, tw) = (x + num_w, w.saturating_sub(num_w + ww + m.inset));
    let name = fit(fb, p.name, m.body_px, tw);
    text::title(fb, tx, top_of(top, line, m.body_px), name, theme::TITLE, m.body_px);
    let says = fit(fb, &p.says, m.small_px, w - num_w);
    let second = top + line;
    text::line(fb, tx, top_of(second, line, m.small_px), says, theme::FOREGROUND, m.small_px);
    if !p.detail.is_empty() {
        let mpx = m.mono_small_px;
        let detail = fit(fb, &p.detail, mpx, w - num_w);
        text::mono(fb, tx, top_of(second + line, line, mpx), detail, theme::MUTED, mpx);
    }
    y + m.proof_row_h()
}
