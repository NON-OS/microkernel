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

//! How long net.core waits for a receive answer: twice what the driver has
//! lately taken, never below the op's floor nor above its ceiling. Pure:
//! net_core_proofs mounts it.
//!
//! A fixed bound fit a fast driver and wrote off every answer of a slow one
//! (QEMU without KVM answers in 26 to 94 ms). The floors keep a fast driver's
//! bound where it was; the ceilings keep a whole poll inside the clients'
//! 64 ms (budget.rs proves it).

/// One receive op's limits, in milliseconds.
pub struct OpBound {
    pub floor: u64,
    pub ceiling: u64,
}

/// OP_RX_BATCH: a batch of up to 44 frames.
pub const BATCH: OpBound = OpBound { floor: 24, ceiling: 32 };
/// OP_RX_PACKET: one frame.
pub const SINGLE: OpBound = OpBound { floor: 8, ceiling: 24 };

/// The bound for the next call, from the learned answer time `ewma`.
pub const fn bound(op: &OpBound, ewma: u64) -> u64 {
    let b = ewma.saturating_mul(2);
    if b < op.floor {
        op.floor
    } else if b > op.ceiling {
        op.ceiling
    } else {
        b
    }
}

/// What one call teaches: its answer time, or with no answer the time it
/// waited, since the answer takes at least that.
pub const fn sample(answered: bool, elapsed: u64, waited: u64) -> u64 {
    if answered || elapsed > waited {
        elapsed
    } else {
        waited
    }
}

/// The learned answer time after `sample`: an eighth of the way to it,
/// rounded up so a run of slow answers always moves it.
pub const fn learn(ewma: u64, sample: u64) -> u64 {
    (ewma.saturating_mul(7).saturating_add(sample).saturating_add(7)) / 8
}
