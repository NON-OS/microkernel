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

//! Starting a circuit, and finding its last hop.

extern crate alloc;

use alloc::vec::Vec;

use crate::path::Relay;

use super::circuit::Circuit;
use super::stage::CircuitStage;

impl Circuit {
    pub fn new(id: u32, path: Vec<Relay>) -> Self {
        Self {
            id,
            stage: CircuitStage::Handshaking,
            hops: Vec::new(),
            path,
            delivered_since: 0,
            proven: false,
            failures: 0,
            opened_at: 0,
            owed_digest: None,
        }
    }

    /// `None` while no hop is up.
    pub fn last_hop(&self) -> Option<usize> {
        self.hops.len().checked_sub(1)
    }
}
