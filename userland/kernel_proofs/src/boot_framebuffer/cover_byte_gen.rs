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

//! Framebuffers and memory BARs at the grain of one byte (L79), inside the
//! model's universe: often only a few bytes long, and often placed so a
//! first or last byte sits on a page edge, a byte either side of one, or a
//! byte either side of the other span's edge, so a rule off by one byte is
//! met rather than left to chance.

use super::cover_find::PAGE;
use super::cover_model::{Lcg, END};

/// `at`, or a byte either side of it.
fn jiggle(g: &mut Lcg, at: u64) -> u64 {
    (at + g.below(3)).saturating_sub(1)
}

/// A length of at least one byte: a few bytes a third of the time, else up
/// to `most`.
fn length(g: &mut Lcg, most: u64) -> u64 {
    if g.below(3) == 0 {
        1 + g.below(4)
    } else {
        1 + g.below(most)
    }
}

/// A framebuffer of at least one byte: its first byte, and separately its
/// end, each near a page edge half the time.
pub(super) fn frame(g: &mut Lcg) -> (u64, u64) {
    let mut base = g.below(END - 1);
    if g.below(2) == 0 {
        base = jiggle(g, base / PAGE * PAGE).min(END - 2);
    }
    let mut end = (base + length(g, 12 * PAGE)).min(END);
    if g.below(2) == 0 {
        end = jiggle(g, end.div_ceil(PAGE) * PAGE).clamp(base + 1, END);
    }
    (base, end - base)
}

/// A memory BAR of at least one byte, four times in five ending at the
/// frame's first byte or its first page, or starting at the frame's end or
/// its last page's end, each give or take a byte.
pub(super) fn bar(g: &mut Lcg, fb: (u64, u64)) -> (u64, u64) {
    let size = length(g, 6 * PAGE);
    let (lo, hi) = (fb.0, fb.0 + fb.1);
    let (page_lo, page_hi) = (lo / PAGE * PAGE, hi.div_ceil(PAGE) * PAGE);
    let at = match g.below(5) {
        0 => lo.saturating_sub(size),
        1 => hi,
        2 => page_lo.saturating_sub(size),
        3 => page_hi,
        _ => g.below(END),
    };
    (jiggle(g, at).min(END - size), size)
}
