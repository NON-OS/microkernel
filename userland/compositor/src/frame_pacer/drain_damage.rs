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

//! One frame's damage, shown rectangle by rectangle.
//!
//! A rectangle leaves the accumulator when it is drained, before it is shown.
//! When the display refused it (a gfx driver call that timed out under load,
//! a present the kernel turned down) the frame used to end there with that
//! rectangle gone: composed into the canvas but never on the screen, it
//! stayed stale there until other damage happened to cover it or the next
//! periodic full frame, seconds later. It goes back into the damage now, so
//! the next frame shows it again.

use crate::state::damage::{DamageAccumulator, Rect};

/// Drain `damage(ctx)`, handing each rectangle to `show`. Stops at the first
/// error, with the refused rectangle damaged again for the next frame.
pub fn drain_damage<C, E>(
    ctx: &mut C,
    damage: fn(&mut C) -> &mut DamageAccumulator,
    mut show: impl FnMut(&mut C, Rect) -> Result<(), E>,
) -> Result<(), E> {
    while let Some(rect) = damage(ctx).drain() {
        if let Err(e) = show(ctx, rect) {
            damage(ctx).accumulate(rect);
            return Err(e);
        }
    }
    Ok(())
}
