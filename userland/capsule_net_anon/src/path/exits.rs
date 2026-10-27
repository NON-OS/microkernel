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

//! What an exit may carry: its summary, and what it refused this boot.

use super::ports::Ports;
use super::refused::Refused;
use super::relay::Relay;

/// What the exits in hand let out, and the ports exits refused this boot.
#[derive(Default)]
pub struct Exits {
    /// Replaced with the relays at every join.
    pub ports: Ports,
    /// Kept for the boot, across refreshes of the consensus.
    pub refused: Refused,
}

impl Exits {
    /// Whether `exit` may carry a stream to `port`: it lets the port out, and
    /// it has not refused the port this boot.
    pub fn takes(&self, exit: &Relay, port: u16) -> bool {
        self.ports.lets_out(exit, port) && !self.refused.holds(&exit.rsa_identity, port)
    }
}
