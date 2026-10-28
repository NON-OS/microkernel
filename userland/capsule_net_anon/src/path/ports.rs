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

//! What each exit in hand lets out beyond the web ports.

extern crate alloc;

use alloc::vec::Vec;

use crate::directory::microdesc::policy::exits_to;
use crate::directory::microdesc::summary::Summary;

use super::relay::Relay;

/// The policy summaries of the relays in hand that carry the Exit flag, by
/// RSA identity. Rebuilt with the relays at every join (relays.rs), so a
/// refreshed consensus brings its own.
#[derive(Default)]
pub struct Ports {
    held: Vec<([u8; 20], Summary)>,
}

impl Ports {
    pub fn new(mut held: Vec<([u8; 20], Summary)>) -> Self {
        held.sort_unstable_by(|a, b| a.0.cmp(&b.0));
        Ports { held }
    }

    /*
     * Ports 80 and 443 keep the rule they had, `exits_web`, both web ports,
     * so the exits install traffic is drawn from do not change. Any other
     * port is read from the summary, where an exit that lets out the web and
     * little else (Tor's reduced exit policy) would end the stream with
     * EXITPOLICY. A relay with no summary held lets such a port out nowhere.
     */
    /// Whether `exit` lets a stream to `port` out.
    pub fn lets_out(&self, exit: &Relay, port: u16) -> bool {
        match port {
            80 | 443 => exit.exits_web,
            _ => self.get(&exit.rsa_identity).is_some_and(|summary| exits_to(summary, port)),
        }
    }

    fn get(&self, identity: &[u8; 20]) -> Option<&Summary> {
        let at = self.held.binary_search_by(|(id, _)| id.cmp(identity)).ok()?;
        Some(&self.held[at].1)
    }
}
