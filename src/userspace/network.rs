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

//! The network stack capsules: links, addresses, transports and the private networks.

#[path = "capsule_net_anon/mod.rs"]
pub mod capsule_net_anon;
#[path = "capsule_net_core/mod.rs"]
pub mod capsule_net_core;
#[path = "capsule_net_dhcp/mod.rs"]
pub mod capsule_net_dhcp;
#[path = "capsule_net_dns/mod.rs"]
pub mod capsule_net_dns;
#[path = "capsule_net_ip/mod.rs"]
pub mod capsule_net_ip;
#[path = "capsule_net_l2/mod.rs"]
pub mod capsule_net_l2;
#[cfg(feature = "nonos-capsule-net-ntp")]
#[path = "capsule_net_ntp/mod.rs"]
pub mod capsule_net_ntp;
#[path = "capsule_net_nym/mod.rs"]
pub mod capsule_net_nym;
#[path = "capsule_net_sockets/mod.rs"]
pub mod capsule_net_sockets;
#[path = "capsule_net_tcp/mod.rs"]
pub mod capsule_net_tcp;
#[path = "capsule_net_udp/mod.rs"]
pub mod capsule_net_udp;
#[path = "capsule_socks5/mod.rs"]
pub mod capsule_socks5;
