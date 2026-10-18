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

//! One frame's damage, handed to the display in batches. A driver that
//! flips at the vertical blank makes each FLUSH wait for one, so a frame
//! sent a rectangle at a time took a blank per rectangle; the wire takes
//! up to 15 in a FLUSH (section 12), and a frame goes in as few as that
//! allows. A batch the display refused is damaged again whole, so the next
//! frame shows every rectangle in it.

use crate::state::damage::{DamageAccumulator, Rect};

pub const BATCH: usize = 15;

/// Drain `damage(ctx)`: `prepare` composes each rectangle and names what
/// to show for it (None: nothing visible), `send` shows a batch. Stops at
/// the first refusal, with that batch's rectangles damaged again.
pub fn drain_batched<C, E>(
    ctx: &mut C,
    damage: fn(&mut C) -> &mut DamageAccumulator,
    mut prepare: impl FnMut(&mut C, Rect) -> Option<Rect>,
    mut send: impl FnMut(&mut C, &[Rect]) -> Result<(), E>,
) -> Result<(), E> {
    let zero = Rect { x: 0, y: 0, width: 0, height: 0 };
    let (mut drawn, mut shown) = ([zero; BATCH], [zero; BATCH]);
    let mut n = 0;
    loop {
        let next = damage(ctx).drain();
        if let Some(rect) = next {
            if let Some(r) = prepare(ctx, rect) {
                (drawn[n], shown[n]) = (rect, r);
                n += 1;
            }
        }
        if n == BATCH || (next.is_none() && n > 0) {
            if let Err(e) = send(ctx, &shown[..n]) {
                for r in &drawn[..n] {
                    damage(ctx).accumulate(*r);
                }
                return Err(e);
            }
            n = 0;
        }
        if next.is_none() {
            return Ok(());
        }
    }
}
