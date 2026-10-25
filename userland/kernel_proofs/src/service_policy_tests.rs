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

//! Reaching a service that carries traffic off the machine takes Network: the
//! policy the registry applies when a service registers its endpoint.

use crate::capabilities::{bit_of, Capability};
use crate::service_policy::required_caps;

#[test]
fn the_anonymity_services_take_network_like_the_stack() {
    let (ipc, network) = (bit_of(Capability::IPC), bit_of(Capability::Network));
    for name in ["net.anon", "net.nym", "net.socks5", "net.tcp", "net.sockets", "net.dns"] {
        assert_eq!(required_caps(name, ipc), ipc | network, "{name}");
    }
}

#[test]
fn a_local_service_takes_only_what_it_asked() {
    let ipc = bit_of(Capability::IPC);
    for name in ["vfs", "app.about", "net.anon.x", "net.ano", "Net.anon", "net.socks", ""] {
        assert_eq!(required_caps(name, ipc), ipc, "{name:?}");
    }
}

#[test]
fn the_keyring_takes_keyring_and_nothing_else_does() {
    let (ipc, keyring) = (bit_of(Capability::IPC), bit_of(Capability::Keyring));
    assert_eq!(required_caps("keyring", ipc), ipc | keyring);
    for name in ["keyring.x", "Keyring", "keyrin", "vfs", "net.nym"] {
        assert_eq!(required_caps(name, ipc) & keyring, 0, "{name:?}");
    }
}
