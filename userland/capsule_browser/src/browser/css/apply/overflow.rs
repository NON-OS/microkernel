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

use crate::browser::css::calc::split_top::words;
use crate::browser::css::computed::{Computed, Overflow};

/* One overflow keyword. overlay is the legacy spelling of auto; the
 * CSS-wide initial, unset and revert all give visible, since overflow is
 * not inherited. inherit is not read, and the declaration drops. */
fn keyword(w: &str) -> Option<Overflow> {
    Some(match w.to_ascii_lowercase().as_str() {
        "visible" | "initial" | "unset" | "revert" | "revert-layer" => Overflow::Visible,
        "hidden" => Overflow::Hidden,
        "clip" => Overflow::Clip,
        "auto" | "overlay" => Overflow::Auto,
        "scroll" => Overflow::Scroll,
        _ => return None,
    })
}

/// overflow (one value for both axes, or x then y), overflow-x, overflow-y.
pub(super) fn apply_overflow(c: &mut Computed, name: &str, value: &str) -> bool {
    let mut it = words(value).map(keyword);
    match (name, it.next(), it.next(), it.next()) {
        ("overflow", Some(Some(x)), None, None) => (c.overflow_x, c.overflow_y) = (x, x),
        ("overflow", Some(Some(x)), Some(Some(y)), None) => (c.overflow_x, c.overflow_y) = (x, y),
        ("overflow-x", Some(Some(x)), None, None) => c.overflow_x = x,
        ("overflow-y", Some(Some(y)), None, None) => c.overflow_y = y,
        ("overflow" | "overflow-x" | "overflow-y", _, _, _) => {}
        _ => return false,
    }
    true
}

impl Computed {
    /// Content past the left and right padding edges is cut off: hidden,
    /// clip, and the scrolling values, since a box cannot scroll sideways
    /// here and would otherwise spill over its neighbours.
    pub fn clips_x(&self) -> bool {
        self.overflow_x != Overflow::Visible
    }

    /// Content past the bottom padding edge is cut off: only hidden and
    /// clip. A scrolling box cannot scroll here, so its content extends the
    /// page instead of being lost.
    pub fn clips_y(&self) -> bool {
        matches!(self.overflow_y, Overflow::Hidden | Overflow::Clip)
    }

    /// Hand this box's overflow to the viewport, which never clips: the box
    /// itself then shows its overflow, as CSS does for the root and body.
    pub fn give_overflow_to_viewport(&mut self) {
        (self.overflow_x, self.overflow_y) = (Overflow::Visible, Overflow::Visible);
    }

    /// Whether either axis is anything but visible.
    pub fn overflow_set(&self) -> bool {
        self.overflow_x != Overflow::Visible || self.overflow_y != Overflow::Visible
    }
}
