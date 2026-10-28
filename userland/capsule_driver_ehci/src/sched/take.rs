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

//! Taking a QH back. An async QH is unlinked from the ring, the async
//! advance doorbell rung so the controller holds no copy of it
//! (advance.rs), then its overlay is emptied (or its characteristics
//! rewritten) and it is linked back; one the controller may still hold is
//! linked back untouched. A periodic QH is parked (park.rs). A halted QH
//! is skipped by the controller, so it is emptied at once.

use super::advance::{doorbell, Advance, Dead};
use super::layout::{qh, ring_prev};
use super::qh::{empty, qh_link};
use crate::env::{Clock, Mem, Regs};

/// Ring QH `i` taken back: emptied (the toggle kept or reset) and, when
/// characteristics and capabilities are given, set for another endpoint.
/// Err once the doorbell went unanswered; the QH is rewritten only on a
/// controller that answered it or was seen to halt.
pub fn take_back<M: Mem, R: Regs, C: Clock>(
    (m, r, c): (&M, &mut R, &C),
    op: u32,
    i: usize,
    keep_dt: bool,
    set: Option<(u32, u32)>,
) -> Result<(), Dead> {
    let (this, prev) = (qh(i), ring_prev(i));
    m.wr(prev, m.rd(this));
    let advance = doorbell(r, c, op);
    if advance != Advance::Wedged {
        if let Some((ch, caps)) = set {
            m.wr(this + 4, ch);
            m.wr(this + 8, caps);
        }
        empty(m, this, keep_dt);
    }
    m.wr(prev, qh_link(m, this));
    match advance {
        Advance::Seen => Ok(()),
        _ => Err(Dead),
    }
}

/// A halted QH emptied, the toggle kept or reset.
pub fn clear_halt<M: Mem>(m: &M, at: usize, keep_dt: bool) {
    empty(m, at, keep_dt);
}
