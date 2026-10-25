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

//! The framebuffers and BARs the seeded sweep tries (L79), inside the
//! model's universe, at steps of a quarter page and sometimes half that.

use super::cover_find::{MMIO, NONE, PAGE, PIO};
use super::cover_model::{Lcg, END, STEP};
use super::cover_rule::Span;

/// A framebuffer of at least one byte, inside the universe.
pub(super) fn frame(g: &mut Lcg) -> (u64, u64) {
    let base = g.below(END / STEP - 1);
    let len = 1 + g.below((END / STEP - base).min(40));
    (base * STEP, len * STEP - g.below(2) * (STEP / 2))
}

/// A BAR of any kind and size, often placed against one of the frame's
/// edges or its pages' edges, so the boundaries are tried and not left to
/// chance.
pub(super) fn bar(g: &mut Lcg, fb: (u64, u64)) -> Span {
    let kind = [MMIO, MMIO, MMIO, PIO, NONE][g.below(5) as usize];
    let size = if g.below(6) == 0 { 0 } else { (1 + g.below(24)) * STEP };
    let (lo, hi) = (fb.0 / PAGE * PAGE, (fb.0 + fb.1).div_ceil(PAGE) * PAGE);
    let edges = [fb.0.checked_sub(size), Some(fb.0 + fb.1), lo.checked_sub(size), Some(hi)];
    let at = edges[g.below(4) as usize].filter(|&b| g.below(2) == 0 && b + size <= END);
    (at.unwrap_or_else(|| g.below((END - size) / STEP + 1) * STEP), size, kind)
}
