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

//! An isoch IN batch: TDs of one service interval's payload each, queued
//! into one half of the pipe's buffer. Pure, so the host proofs hold it.

use crate::protocol::{ISOCH_IN_MAX, ISOCH_PACKETS_MAX};

/// As for an OUT batch: a second is a stream that has stopped.
pub const ISOCH_TIMEOUT_MS: u64 = 1_000;
/// Each TD's completion code and residual, as its event gave them.
pub type Got = [(u8, u32); ISOCH_PACKETS_MAX];

/// A batch on the ring: each TD's TRB address, the half its bytes land in,
/// and the bytes each TD may take.
#[derive(Clone, Copy)]
pub struct InBatch {
    pub tds: [u64; ISOCH_PACKETS_MAX],
    pub n: usize,
    pub half: usize,
    pub td_len: u32,
}

/// Whether `n` TDs of `td_len` bytes make a batch: 1 to ISOCH_PACKETS_MAX of
/// them, whose bytes fit one reply.
pub fn in_batch_fits(n: usize, td_len: u32) -> bool {
    (1..=ISOCH_PACKETS_MAX).contains(&n) && td_len != 0 && n * td_len as usize <= ISOCH_IN_MAX
}
