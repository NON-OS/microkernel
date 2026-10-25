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

//! The chain's word on each channel, from settle: every
//! channel at start, and a channel about to be charged when its copy is
//! older than a minute. Best effort: without settle the meter carries on
//! from its cached state and the local floor.

use nonos_libc::{mk_time_millis, mk_uptime_ms};
use nonos_nox_meter::timeout::still_fresh;

use super::hold::Meter;
use crate::clients::{channel_state, ROUTED_MS};

/// A charge does not wait on the network: a refresh before one is short.
const CHARGE_MS: u64 = 1_500;

/// The wall clock, for what is compared with chain time.
pub(crate) fn now_ms() -> u64 {
    mk_time_millis().max(0) as u64
}

/// The monotonic clock, for timeouts: STEP_MS and STALE_MS (timeout.rs).
pub(crate) fn uptime_ms() -> u64 {
    mk_uptime_ms().max(0) as u64
}

impl Meter {
    pub(super) fn refresh_all(&mut self) {
        let ids: alloc::vec::Vec<([u8; 32], u64)> =
            self.state.channels.iter().map(|c| (c.id, c.chain_id)).collect();
        for (id, chain) in ids {
            self.refresh(&id, chain, ROUTED_MS);
        }
    }

    /// Refresh the channels a charge to this payee and listing could land
    /// in, those settle has not reported in the last minute.
    pub fn refresh_stale(&mut self, payee: &[u8; 20], listing: &[u8; 32], chain: u64) {
        let now = uptime_ms();
        let stale: alloc::vec::Vec<[u8; 32]> = self
            .state
            .channels
            .iter()
            .filter(|c| &c.payee == payee && &c.listing_id == listing && c.chain_id == chain)
            .filter(|c| !self.seen.iter().any(|(id, at)| id == &c.id && still_fresh(*at, now)))
            .map(|c| c.id)
            .collect();
        for id in stale {
            self.refresh(&id, chain, CHARGE_MS);
        }
    }

    fn refresh(&mut self, id: &[u8; 32], chain: u64, timeout_ms: u64) {
        let Some(view) = channel_state(chain, id, timeout_ms) else { return };
        let next = self.state.from_chain(id, &view);
        self.note(next);
        self.seen.retain(|(seen, _)| seen != id);
        self.seen.push((*id, uptime_ms()));
    }
}
