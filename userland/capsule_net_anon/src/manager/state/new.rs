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

use super::super::refresh_rule::usable;
use super::bootstrap::Bootstrap;
use super::manager::Manager;

impl Manager {
    pub fn new(tcp_port: u32) -> Self {
        Self {
            tcp_port,
            bootstrap: Bootstrap::Cold,
            retry_after: 0,
            certs: alloc::vec::Vec::new(),
            refetch: alloc::vec::Vec::new(),
            entries: alloc::vec::Vec::new(),
            micro: alloc::vec::Vec::new(),
            dir: Default::default(),
            relays: alloc::vec::Vec::new(),
            weights: Weights::default(),
            valid_after: 0,
            fresh_until: 0,
            valid_until: 0,
            consensus_signatures: 0,
            srv_current: None,
            srv_previous: None,
            onion: alloc::vec::Vec::new(),
            desc_cache: Default::default(),
            client_keys: alloc::vec::Vec::new(),
            names: Default::default(),
            link: None,
            circuits: alloc::vec::Vec::new(),
            streams: alloc::vec::Vec::new(),
            next_circuit: 1,
            next_stream: 1,
            authority_cursor: 0,
            circuit_cursor: 0,
            guard: None,
            given_up: alloc::vec::Vec::new(),
            refreshing: false,
            next: None,
        }
    }

    /// Whether the relay set may still be used to build a circuit.
    ///
    pub fn usable_at(&self, now: u64) -> bool {
        let ready = self.bootstrap == Bootstrap::Ready;
        usable(ready, self.refreshing, self.relays.len(), now, self.valid_until)
    }
}
