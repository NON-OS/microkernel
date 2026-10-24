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

//! How a hostile sink behaves on one run: how often each kind of AUX reply
//! comes, how its link status moves, and whether its transmitter starts.

use super::rng::Rng;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    /// Clock recovery and equalisation done after this many reads.
    Settles(u32),
    /// Asks for more swing every read and never locks.
    Never,
    /// Asks for swing 0, then 1, then 0 again, never locking.
    Flips,
    /// Any bytes at all, lanes it was not asked to train among them.
    Noise,
}

/// Percent of transfers for each answer that is not a clean ACK.
#[derive(Clone, Copy, Debug)]
pub struct Profile {
    pub timeout: u32,
    pub defer: u32,
    pub nack: u32,
    pub undefined: u32,
    pub empty: u32,
    /// One in this many ACKs comes short, and as many run long.
    pub odd_len: u32,
    pub status: Status,
    pub start_fails: bool,
}

pub fn profile(r: &mut Rng) -> Profile {
    let rare = |r: &mut Rng| [0, 0, 1, 3, 20, 100][r.below(6) as usize];
    let status = match r.below(4) {
        0 => Status::Settles(r.below(12)),
        1 => Status::Never,
        2 => Status::Flips,
        _ => Status::Noise,
    };
    Profile {
        timeout: rare(r),
        defer: rare(r),
        nack: rare(r) / 4,
        undefined: rare(r) / 4,
        empty: rare(r) / 4,
        odd_len: [0, 2, 4, 16][r.below(4) as usize],
        status,
        start_fails: r.below(10) == 0,
    }
}
