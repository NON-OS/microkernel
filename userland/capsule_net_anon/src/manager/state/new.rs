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

//! A manager with nothing fetched and nothing open.

use crate::path::Weights;

use super::bootstrap::Bootstrap;
use super::manager::Manager;

impl Manager {
    pub fn new(tcp_port: u32) -> Self {
        Self {
            tcp_port,
            bootstrap: Bootstrap::Cold,
            retry_after: 0,
            certs: alloc::vec::Vec::new(),
            entries: alloc::vec::Vec::new(),
            micro: alloc::vec::Vec::new(),
            micro_cursor: 0,
            relays: alloc::vec::Vec::new(),
            weights: Weights::default(),
            fresh_until: 0,
            valid_until: 0,
            link: None,
            circuits: alloc::vec::Vec::new(),
            streams: alloc::vec::Vec::new(),
            next_circuit: 1,
            next_stream: 1,
            authority_cursor: 0,
            circuit_cursor: 0,
            guard: None,
        }
    }

    /// Whether the relay set may still be used to build a circuit.
    ///
    pub fn usable_at(&self, now: u64) -> bool {
        self.bootstrap == Bootstrap::Ready && !self.relays.is_empty() && now < self.valid_until
    }
}
