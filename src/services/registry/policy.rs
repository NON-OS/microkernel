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

/*
 * The keyring answers each caller only for the keys it owns, by the pid the
 * kernel stamps, so no capsule can read another's. Reaching it still takes
 * Keyring: a capsule granted none has no business making the keyring hold a
 * secret for it, and the bit means what a manifest says it means only when
 * the kernel holds a sender to it.
 */
const KEY_SERVICES: [&str; 1] = ["keyring"];

/*
 * The two fronts that carry only anonymity streams, Anyone's and the Nym
 * mixnet's. A holder of ForeignExec, the Linux personality, reaches these
 * without Network and no other network service: each front refuses such a
 * caller unless the caller's guest family holds a grant it made
 * (userland/linux_net_grant), and the rest of NETWORK_SERVICES, every one
 * that could open a direct socket, stays closed to it. So a guest's streams
 * leave only over Nym or Anyone by this rule, whatever the personality does
 * (userland/capsule_linux/design/network-grant.md).
 */
pub const ANON_FRONTS: [&str; 2] = ["net.anon", "net.socks5"];

/// Whether a caller holding `held` may send to `name`, whose endpoint asks
/// `required`, as a ForeignExec holder without Network: only to an
/// anonymity front, and only with everything else it asks.
pub fn admits_foreign(name: &str, required: u64, held: u64) -> bool {
    let network = Capability::Network.bit();
    ANON_FRONTS.contains(&name)
        && required & network != 0
        && held & Capability::ForeignExec.bit() != 0
        && held & (required & !network) == required & !network
}

pub fn required_caps(name: &str, default: u64) -> u64 {
    if NETWORK_SERVICES.contains(&name) {
        return default | Capability::Network.bit();
    }
    if KEY_SERVICES.contains(&name) {
        return default | Capability::Keyring.bit();
    }
    default
}
