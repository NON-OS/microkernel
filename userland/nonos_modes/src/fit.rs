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

//! The display wire's rule for the mode to ask for when the host wants a
//! size (PLAN-coverage section 12, "Selection"): nothing for a zero size
//! or the size already set; an adapter taking any size within its range
//! is clamped to it; an adapter with a list gets its largest mode that
//! fits inside the wanted size.

/// A MODE_RANGE adapter's limits, from its CAPS reply.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Range {
    pub max_width: u32,
    pub max_height: u32,
    /// Widths come in multiples of this (at least 1).
    pub width_align: u32,
    /// Bytes for the largest frame plus the cursor slot.
    pub budget: u64,
}

/// The smallest size a range adapter is asked for.
pub const MIN: (u32, u32) = (640, 480);
/// The hardware cursor's slot, 64 x 64 pixels of four bytes, which the
/// surface budget also holds.
pub const CURSOR_SLOT: u64 = 64 * 64 * 4;

pub fn fit(
    want: (u32, u32),
    current: (u32, u32),
    range: Option<&Range>,
    modes: &[(u32, u32)],
) -> Option<(u32, u32)> {
    if want.0 == 0 || want.1 == 0 {
        return None;
    }
    let got = match range {
        Some(r) => clamp(want, r)?,
        None => *modes
            .iter()
            .filter(|m| m.0 <= want.0 && m.1 <= want.1)
            .max_by_key(|m| (m.0 as u64 * m.1 as u64, m.0))?,
    };
    (got != current).then_some(got)
}

/// Clamped to the adapter's range, the width cut to its alignment, and
/// cut again, keeping the shape, while the frame overruns the budget.
fn clamp(want: (u32, u32), r: &Range) -> Option<(u32, u32)> {
    let align = r.width_align.max(1);
    if r.max_width < MIN.0 || r.max_height < MIN.1 {
        return None;
    }
    let (mut w, mut h) = (want.0.clamp(MIN.0, r.max_width), want.1.clamp(MIN.1, r.max_height));
    let frame = |w: u32, h: u32| w as u64 * h as u64 * 4 + CURSOR_SLOT;
    while frame(w, h) > r.budget && w > MIN.0 && h > MIN.1 {
        (w, h) = ((w * 15 / 16).max(MIN.0), (h * 15 / 16).max(MIN.1));
    }
    w = w / align * align;
    let fits = w >= MIN.0 && w <= r.max_width && h <= r.max_height;
    (fits && frame(w, h) <= r.budget).then_some((w, h))
}
