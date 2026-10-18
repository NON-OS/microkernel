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

//! A wire driver in miniature for the batch proofs: the FLUSHes it took,
//! one refused when asked, and a frame drained through the real batching.

use crate::damage::{DamageAccumulator, Rect};
use crate::drain_batch::drain_batched;

pub(crate) struct Display {
    pub(crate) damage: DamageAccumulator,
    pub(crate) refuse: usize,
    pub(crate) flushes: Vec<Vec<(u32, u32)>>,
}

pub(crate) fn display(rects: &[(u32, u32)]) -> Display {
    let mut d = Display { damage: DamageAccumulator::new(), refuse: 0, flushes: Vec::new() };
    for &(x, y) in rects {
        d.damage.accumulate(Rect { x, y, width: 4, height: 4 });
    }
    d
}

pub(crate) fn send(d: &mut Display, rects: &[Rect]) -> Result<(), ()> {
    if d.refuse > 0 {
        d.refuse -= 1;
        return Err(());
    }
    d.flushes.push(rects.iter().map(|r| (r.x, r.y)).collect());
    Ok(())
}

pub(crate) fn frame(d: &mut Display, visible: impl Fn(&Rect) -> bool) -> Result<(), ()> {
    drain_batched(d, |d| &mut d.damage, |_, r| visible(&r).then_some(r), send)
}
