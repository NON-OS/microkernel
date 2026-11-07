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
//! The most a client gets through the sender at a profile, for the status op
//! and the Settings line that names it. Pure: the packet sizes come in.
//!
//! Outbound, one message packet leaves a sending tick. Inbound, each reply
//! packet spends one reply block, and blocks go out in top up packets on the
//! same clock, so one tick of top up buys as many replies as blocks fit in a
//! packet. An upper bound: acknowledgements and the blocks a request carries
//! are left out.

use super::cover_clock::Profile;

/// KiB a second out and in, rounded down.
pub fn ceilings(profile: Profile, per_packet: u64, blocks_per_packet: u64) -> (u16, u16) {
    let mean = profile.send_mean_ns.max(1);
    let out = per_packet.saturating_mul(1_000_000_000) / mean;
    let inbound = out.saturating_mul(blocks_per_packet);
    let kib = |b: u64| u16::try_from(b / 1024).unwrap_or(u16::MAX);
    (kib(out), kib(inbound))
}
