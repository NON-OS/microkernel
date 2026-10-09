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


//! Everything a client derives before it asks for a descriptor: the period,
//! the blinded key, the subcredential and the HSDirs to ask (pick_hsdir_v3
//! and node_set_hsdir_index's fetch index in the fork).

extern crate alloc;

use alloc::vec::Vec;

use super::blind::{blinded_key, subcredential};
use super::period::{Clock, PERIOD_MINUTES};
use super::ring::{disaster_srv, node_index, responsible};

/// One service in one time period.
#[derive(Clone)]
pub struct Lookup {
    pub identity: [u8; 32],
    pub blinded: [u8; 32],
    pub subcredential: [u8; 32],
    pub period: u64,
}

impl Lookup {
    /// `None` on a clock that names no period or a key off the curve.
    pub fn new(identity: &[u8; 32], clock: &Clock) -> Option<Self> {
        let period = clock.period()?;
        let blinded = blinded_key(identity, period, PERIOD_MINUTES)?;
        Some(Self { identity: *identity, blinded, subcredential: subcredential(identity, &blinded), period })
    }

    /// The responsible HSDirs, as positions into `hsdirs`, each an HSDir's
    /// Ed25519 identity. A fetch uses the current shared random value or the
    /// previous one depending on where `clock` sits, and the disaster value
    /// for the period when the consensus published neither.
    pub fn hsdirs(
        &self,
        clock: &Clock,
        srv_current: Option<[u8; 32]>,
        srv_previous: Option<[u8; 32]>,
        hsdirs: &[[u8; 32]],
    ) -> Option<Vec<usize>> {
        let chosen = if clock.current_srv()? { srv_current } else { srv_previous };
        let srv = chosen.unwrap_or_else(|| disaster_srv(self.period, PERIOD_MINUTES));
        let indexes: Vec<[u8; 32]> =
            hsdirs.iter().map(|id| node_index(id, &srv, self.period, PERIOD_MINUTES)).collect();
        Some(responsible(&indexes, &self.blinded, self.period, PERIOD_MINUTES))
    }
}
