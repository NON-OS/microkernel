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

//! What the reader is told when a network refuses a request.

use super::choice::Network;

impl Network {
    /// What a refused SOCKS CONNECT means on this network, from its reply
    /// code. The same code names a different cause on each: Anyone answers
    /// 3 while it has no directory, guard link or circuit yet, and 4 when
    /// the exit could not resolve the name.
    pub fn refused(self, rep: u8) -> &'static str {
        match (self, rep) {
            (Network::Anyone, 0x01) => "anyone: the stream could not be opened",
            (Network::Anyone, 0x02) => "anyone: refused by the exit's policy",
            (Network::Anyone, 0x03) => "anyone: not connected yet, no directory or circuit",
            (Network::Anyone, 0x04) => "anyone: the exit could not resolve the host",
            (Network::Anyone, 0x05) => "anyone: the host refused the connection",
            (Network::Anyone, 0x06) => "anyone: the exit timed out reaching the host",
            (_, 0x01) => "mixnet: the request could not be built",
            (_, 0x02) => "mixnet: refused by ruleset",
            (_, 0x03) => "mixnet: no session, the mixnet is not connected",
            (_, 0x04) => "mixnet: no exit for this destination",
            (_, 0x05) => "mixnet: the gateway refused the request",
            (_, 0x06) => "mixnet: expired in transit",
            _ => "socks connect rejected",
        }
    }

    /// Why a request was refused when the service is not registered.
    pub fn absent(self) -> &'static str {
        match self {
            Network::Direct => "direct: net.sockets is not running",
            Network::Nym => "mixnet: net.socks5 is not running, so nothing was sent",
            Network::Anyone => "anyone: net.anon is not running, so nothing was sent",
        }
    }
}
