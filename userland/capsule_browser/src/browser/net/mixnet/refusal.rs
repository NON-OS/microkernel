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
            (Network::Anyone, 0x01) => {
                "The Anyone network could not open a stream to the host. Try again later."
            }
            (Network::Anyone, 0x02) => "The Anyone exit's policy does not allow that destination.",
            (Network::Anyone, 0x03) => {
                "The Anyone network is not connected yet: it has no directory or circuit. Try \
                 again in a minute."
            }
            (Network::Anyone, 0x04) => {
                "The Anyone exit could not find an address for the host. Check the spelling of \
                 the address."
            }
            (Network::Anyone, 0x05) => "The host refused the connection from the Anyone exit.",
            (Network::Anyone, 0x06) => {
                "The Anyone exit timed out reaching the host. It may be down; try again later."
            }
            (_, 0x01) => "The Nym mixnet could not build the request for this address.",
            (_, 0x02) => "The Nym exit's rules refuse that destination.",
            (_, 0x03) => {
                "The Nym mixnet is not connected yet: net.socks5 has no session. Try again in a \
                 minute."
            }
            (_, 0x04) => "The Nym mixnet has no exit for this destination.",
            (_, 0x05) => "The Nym gateway refused the request. Try again later.",
            (_, 0x06) => "The request expired crossing the Nym mixnet. Try again.",
            _ => "socks connect rejected",
        }
    }

    /// Why a request was refused when the service is not registered.
    pub fn absent(self) -> &'static str {
        match self {
            Network::Direct => "net.sockets is not running, so there is no direct network.",
            Network::Nym => {
                "The Nym mixnet is chosen, but its service net.socks5 is not running, so nothing \
                 was sent. Choose another network in the browser's settings, or try again once \
                 it has started."
            }
            Network::Anyone => {
                "The Anyone network is chosen, but its service net.anon is not running, so \
                 nothing was sent. Choose another network in the browser's settings, or try \
                 again once it has started."
            }
        }
    }
}
