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

//! Binding a gateway: its sessions, and the key that belongs to its link.

use super::super::Gateway;
use super::types::Table;

impl Table {
    /// Bind a gateway, dropping sessions only when it is a different one.
    ///
    /// Sessions are keyed to the gateway that carries them, so a change has
    /// to clear them. Rebinding the same gateway does not, and clearing
    /// regardless meant a client that opened a session could have it taken
    /// away by a reconnect it never asked for.
    pub fn set_gateway(&mut self, gateway: Gateway) -> Option<Gateway> {
        // A key registered on another connection goes with that connection.
        crate::state::keep_gateway_key_for(gateway.link());
        let changed = match self.gateway {
            Some(current) => current.ip != gateway.ip || current.port != gateway.port,
            None => false,
        };
        if changed {
            self.reset_sessions();
        } else {
            // The same gateway on a new connection. Sessions keep their
            // numbers, and the replies already promised to them still come
            // home through this gateway, but each holds a copy of the link it
            // sends on, and the old link is closed: every send went to a dead
            // stream, failed, and dropped the binding again.
            for session in self.sessions.iter_mut() {
                session.gateway = gateway;
            }
        }
        self.gateway.replace(gateway)
    }
}
