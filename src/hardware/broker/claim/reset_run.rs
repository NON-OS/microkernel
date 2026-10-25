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

//! The reset's order, pure, over any ResetIo.

use super::reset_rule::{Reset, ResetIo, PENDING_MS, POLL_MS, READY_MS, SETTLE_MS};

pub(crate) fn reset(io: &mut impl ResetIo) -> Reset {
    let Some(how) = io.flr() else {
        return Reset::NotOffered;
    };
    let pending_left = !waited(io, PENDING_MS, |io| !io.pending(how));
    io.save();
    io.initiate(how);
    io.pause_ms(SETTLE_MS);
    if !waited(io, READY_MS, |io| io.ready()) {
        return Reset::NotReady;
    }
    io.restore();
    Reset::Done { pending_left }
}

/// Whether `done` held within `budget` ms, asked every POLL_MS.
fn waited<R: ResetIo>(io: &mut R, budget: u64, mut done: impl FnMut(&mut R) -> bool) -> bool {
    let start = io.now_ms();
    loop {
        if done(io) {
            return true;
        }
        if io.now_ms().saturating_sub(start) >= budget {
            return false;
        }
        io.pause_ms(POLL_MS);
    }
}
