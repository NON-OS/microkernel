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

/*
 * The least the built-in faces are drawn or measured at (the design notes,
 * Type): nothing under 12, and only the mono face, which `type.label` is set
 * in, under 14. Below that a face renders unreadable on a high-resolution
 * panel. Draw and measure clamp through the same function, so a clamped size
 * stays self-consistent and layout never drifts from what is drawn. Web and
 * page fonts pass their own faces and are deliberately not clamped here.
 */
pub const MIN_UI_PX: f32 = 14.0;
pub const MIN_MONO_PX: f32 = 12.0;

/*
 * The floor is applied first and then the whole thing is scaled, so the chosen
 * text size moves the floor with it. Scaling first would let Large grow a size
 * that was already clamped while leaving the clamped ones behind, and the screen
 * would come out with two different notions of how big text is.
 *
 * Tiny does take the floor below what it guards, deliberately: the floor exists
 * so nothing is unreadable by accident, and someone who picks Tiny has not
 * chosen it by accident.
 */
pub fn readable_px_for(px: f32, mono: bool) -> f32 {
    let floor = if mono { MIN_MONO_PX } else { MIN_UI_PX };
    super::text_scale::apply(if px < floor { floor } else { px })
}

/// The proportional face's size: for line metrics, which are the text face's.
pub fn readable_px(px: f32) -> f32 {
    readable_px_for(px, false)
}
