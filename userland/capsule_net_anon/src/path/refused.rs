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

//! Exits that ended a stream to a port before it connected, this boot.

extern crate alloc;

use alloc::vec::Vec;

/// Refusals remembered. Past this many the oldest is forgotten, so a long
/// session that meets many refusing exits holds a bounded list.
pub const REFUSED_MAX: usize = 64;

/*
 * A summary says what an exit means to let out, not what it does: one whose
 * policy changed since it published, or that cannot reach a network, still
 * ends the stream. Tor marks such an exit as rejecting the address it refused
 * (connection_ap_process_end_not_open); this keeps the exit and the port, and
 * never the host, so nothing here says where a stream was going.
 */
/// Pairs of exit identity and port, oldest first.
#[derive(Default)]
pub struct Refused {
    held: Vec<([u8; 20], u16)>,
}

impl Refused {
    /// Remember that the exit with `identity` ended a stream to `port`.
    pub fn note(&mut self, identity: [u8; 20], port: u16) {
        if self.holds(&identity, port) {
            return;
        }
        if self.held.len() >= REFUSED_MAX {
            self.held.remove(0);
        }
        self.held.push((identity, port));
    }

    /// Whether the exit with `identity` refused `port` this boot.
    pub fn holds(&self, identity: &[u8; 20], port: u16) -> bool {
        self.held.iter().any(|(id, p)| id == identity && *p == port)
    }
}
