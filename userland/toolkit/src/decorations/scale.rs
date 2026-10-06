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

//! The frame at the desktop's display scale.
//!
//! The shell draws its bar, dock and type larger on a larger canvas, by the
//! brand rule in quarters (4 is one to one, 5 is 1.25, 8 is 2). A frame drawn
//! at fixed pixel sizes beside them read as a thin strip with buttons too
//! small to hit. Every frame metric goes through here, rounded the way the
//! shell's `px` rounds, so the title bar, buttons, border and title text grow
//! together with the shell, and the hit test measures what is drawn.

/// The scale at which every metric is its constant: one to one.
pub const ONE: u32 = 4;

/// `v` pixels at `quarters` of scale, to the nearest pixel. Below one to one
/// is never asked for, so a smaller value (a zero from an unknown display)
/// reads as one to one.
pub fn at(v: u32, quarters: u32) -> u32 {
    v.saturating_mul(quarters.max(ONE)).saturating_add(2) / 4
}

/// A type size at `quarters` of scale.
pub fn px_at(px: f32, quarters: u32) -> f32 {
    px * quarters.max(ONE) as f32 / 4.0
}
