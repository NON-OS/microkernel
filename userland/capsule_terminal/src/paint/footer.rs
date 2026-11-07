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

use alloc::vec::Vec;
use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::design::scale::scale;
use nonos_toolkit::design::size::SPACE;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::{draw, Ink, Label, Rect, Separator};

/// The hint bar: what to press, and what it does.
///
/// Two colours rather than one. A single shade for both makes the bar read as
/// one long sentence, which is why a reader stops seeing it at all; the key
/// carries the accent so the eye can find it and skip the rest.
pub const HINTS: &[(&str, &str)] = &[
    ("Tab", "complete"),
    ("|", "pipe"),
    (">", "redirect"),
    ("^W", "word"),
    ("^K", "kill"),
    ("^L", "clear"),
];

/// Height the hint bar needs: one line of body text and `SPACE.xs` above and
/// below it.
pub fn footer_h() -> u32 {
    let pad = scale().px(u32::from(SPACE.xs));
    Label::new("").size().1.saturating_add(pad.saturating_mul(2))
}

/// The key and label boxes of each hint that fits whole in `at`, from
/// `SPACE.lg` in. A key sits `SPACE.xs` from its label and pairs `SPACE.lg`
/// apart, so the pairs group before they read left to right; a pair that would
/// run under the right edge is left off rather than cut.
pub fn hint_boxes(at: Rect) -> Vec<(Rect, Rect)> {
    let s = scale();
    let (pad, near) = (s.px(u32::from(SPACE.lg)), s.px(u32::from(SPACE.xs)));
    let right = at.x.saturating_add(at.w).saturating_sub(pad);
    let mut x = at.x.saturating_add(pad);
    let mut out = Vec::new();
    for (key, label) in HINTS {
        let (kw, lw) = (Label::new(key).size().0, Label::new(label).size().0);
        let end = x.saturating_add(kw).saturating_add(near).saturating_add(lw);
        if end > right {
            break;
        }
        let k = Rect { x, w: kw, ..at };
        out.push((k, Rect { x: end - lw, w: lw, ..at }));
        x = end.saturating_add(pad);
    }
    out
}

/// The bar on the window's chrome under a hairline. While a search is open
/// the search bar's hint takes its place, so the keys are left off.
pub fn draw_footer(fb: &mut PaintBuffer, at: Rect, hints: bool) {
    draw::fill(fb, at, roles().surface);
    Separator::new().draw(fb, Rect { h: scale().hair(), ..at });
    if !hints {
        return;
    }
    for ((key, label), (k, l)) in HINTS.iter().zip(hint_boxes(at)) {
        Label::new(key).ink(Ink::Accent).draw(fb, k);
        Label::new(label).ink(Ink::Muted).draw(fb, l);
    }
}
