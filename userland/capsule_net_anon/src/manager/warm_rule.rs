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

//! The warm circuit: one general circuit drawn ahead and kept for installs,
//! so an install's streams find a path already up, of their own, with its
//! whole life ahead of it. Pure, and held in net_anon_proofs.
//!
//! It is drawn only while the network is up, and only once browsing has an
//! open circuit of its own, so nothing else waits behind it. It counts
//! against CIRCUIT_MAX as every general circuit does. Only a stream marked
//! as an install takes it, and an install takes it before any other. It
//! takes no new stream after CIRCUIT_DIRTY_SECONDS, or after its first
//! failure where a general circuit is given two, and is then retired.

use crate::protocol::{CIRCUIT_DIRTY_SECONDS, CIRCUIT_FAILURES_MAX, CIRCUIT_MAX};

/// Failures that retire the warm circuit: the first.
pub const WARM_FAILURES_MAX: u8 = 1;

/// Whether the next general circuit drawn is the warm one: the network is
/// up, `general` circuits leave room under the cap, no warm one that still
/// takes streams is held, and `others_open` circuits already serve the rest.
pub fn draw_warm(up: bool, general: usize, warm_held: bool, others_open: usize) -> bool {
    up && general < CIRCUIT_MAX && !warm_held && others_open > 0
}

/// Whether a stream may go down a circuit: the warm one only for an install.
pub fn may_take(install: bool, warm: bool) -> bool {
    install || !warm
}

/// The failed streams that retire a circuit.
pub fn failures_max(warm: bool) -> u8 {
    if warm {
        WARM_FAILURES_MAX
    } else {
        CIRCUIT_FAILURES_MAX
    }
}

/// Whether an open circuit takes no more streams: too old, or failed enough.
pub fn spent(warm: bool, failures: u8, opened_at: u64, now: u64) -> bool {
    failures >= failures_max(warm) || now >= opened_at.saturating_add(CIRCUIT_DIRTY_SECONDS)
}

/// Whether a stream failure is charged to its circuit. A general circuit is
/// charged only while it has never delivered (blame.rs says why); the warm
/// one carries installs, and goes at its first failure whatever it carried.
pub fn charged(warm: bool, proven: bool) -> bool {
    warm || !proven
}
