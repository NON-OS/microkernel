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

//! What a SCREENSHOT holder gets: the scene composed again into its surface,
//! with every layer of the trusted band left out, so a prompt or the lock
//! screen is never in a capture. The cursor, the night tint and the privacy
//! band are the compositor's own and are not in it either. Holds no
//! context, so the host proofs capture real scenes with it.

use super::compose::{compose, Scene};
use crate::state::attach::SurfaceKernel;
use crate::state::damage::Rect;
use crate::state::trusted_rule::TRUSTED_Z;
use crate::sw_blitter::Surface;

/// Compose a `canvas`-sized screen into `dst` from its top left corner; the
/// width and height captured. The scene's own ceiling is lowered to the
/// trusted band whatever the caller set.
pub fn capture<K: SurfaceKernel>(
    dst: Surface,
    canvas: (u32, u32),
    mut s: Scene<'_, K>,
) -> (u32, u32) {
    let (w, h) = (canvas.0.min(dst.width), canvas.1.min(dst.height));
    s.ceiling = s.ceiling.min(TRUSTED_Z);
    compose(dst, Rect { x: 0, y: 0, width: w, height: h }, s, None);
    (w, h)
}
