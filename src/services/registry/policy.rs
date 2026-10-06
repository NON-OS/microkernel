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

use crate::capabilities::Capability;

/*
 * Services that carry traffic off the machine. Reaching one takes Network, not
 * only IPC: net.anon, net.nym and net.socks5 carry anonymity streams, which a
 * capsule without Network must not be able to open through them. net.socks5
 * is the SOCKS front of net.nym, so leaving it out left the same streams one
 * IPC call away from a capsule the list was written to keep off them.
 */
const NETWORK_SERVICES: [&str; 11] = [
    "net.core",
    "net.l2",
    "net.ip",
    "net.udp",
    "net.tcp",
    "net.dns",
    "net.dhcp.client",
    "net.sockets",
    "net.nym",
    "net.anon",
    "net.socks5",
];

pub fn required_caps(name: &str, default: u64) -> u64 {
    if NETWORK_SERVICES.contains(&name) {
        return default | Capability::Network.bit();
    }
    default
}
