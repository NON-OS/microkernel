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

use crate::browser::css::computed::Computed;

/* Stacking level, overflow clipping, opacity and the post-layout effects
 * (transform, its origin, clip-path). */
pub(super) fn apply_visual(c: &mut Computed, name: &str, value: &str, fs: u32) -> bool {
    match name {
        "overflow" | "overflow-x" | "overflow-y" => {
            return super::overflow::apply_overflow(c, name, value)
        }
        "opacity" => {
            if let Ok(v) = value.trim().parse::<f32>() {
                if (0.0..=1.0).contains(&v) {
                    c.opacity = (v * 255.0) as u8;
                }
            }
        }
        /* visibility keeps the box's space but paints nothing, which zero
         * opacity models; descendant re-show is rare enough to ignore. */
        "visibility" => match value.trim() {
            "hidden" | "collapse" => c.opacity = 0,
            "visible" => c.opacity = 255,
            _ => {}
        },
        "z-index" => super::z_index::apply_z_index(c, value),
        "transform" => {
            if let Some(t) = super::transform::parse_transform(value, fs) {
                c.fx.transform = t;
            }
        }
        "transform-origin" => {
            if let Some(o) = super::origin::parse_position(value, fs) {
                c.fx.origin = o;
            }
        }
        "clip-path" => {
            if let Some(clip) = super::clip::parse_clip(value, fs) {
                c.fx.clip = clip;
            }
        }
        _ => return false,
    }
    true
}
