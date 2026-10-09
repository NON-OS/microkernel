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

use super::plan::ceil;

/// The raster size for vector art of natural size `nat`: the display hint
/// covered at any scale, up or down, within `max_side` a side; the natural
/// size when no hint is known.
pub(super) fn vector_target(nat: (u32, u32), hint: (u32, u32), max_side: u32) -> (u32, u32) {
    let (w, h) = (nat.0.max(1) as f64, nat.1.max(1) as f64);
    let s = match hint {
        (0, 0) => 1.0,
        (hw, hh) => (hw as f64 / w).max(hh as f64 / h),
    };
    let s = s.min(max_side as f64 / w).min(max_side as f64 / h);
    (ceil(w * s).clamp(1, max_side), ceil(h * s).clamp(1, max_side))
}

/// The raster size for vector art drawn only as an <img> box's content:
/// the box's own w x h, scaled down within `max_side` a side, so the art's
/// preserveAspectRatio decides how it sits in the box (a "none" stretch, a
/// "slice" crop) as when it is drawn at the box's size. Without both sides
/// known it is vector_target.
pub(super) fn box_target(nat: (u32, u32), hint: (u32, u32), max_side: u32) -> (u32, u32) {
    let (hw, hh) = hint;
    if hw == 0 || hh == 0 {
        return vector_target(nat, hint, max_side);
    }
    let s = (max_side as f64 / hw as f64).min(max_side as f64 / hh as f64).min(1.0);
    (ceil(hw as f64 * s).clamp(1, max_side), ceil(hh as f64 * s).clamp(1, max_side))
}
