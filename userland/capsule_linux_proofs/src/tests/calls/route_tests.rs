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

//! Which mirrors are dialled directly whatever network the person chose:
//! only those on private or link-local addresses. Every other mirror,
//! including what only looks close and any name, is reached through the
//! chosen network.

use crate::route::is_local;

#[test]
fn private_and_link_local_mirrors_go_direct() {
    for ip in
        ["10.0.2.2", "10.255.255.255", "172.16.0.1", "172.31.9.9", "192.168.1.10", "169.254.3.4"]
    {
        assert!(is_local(ip), "{ip}");
    }
}

#[test]
fn public_addresses_follow_the_chosen_network() {
    for ip in [
        "151.101.66.132",
        "172.15.0.1",
        "172.32.0.1",
        "192.169.0.1",
        "169.255.0.1",
        "11.0.0.1",
        "8.8.8.8",
    ] {
        assert!(!is_local(ip), "{ip}");
    }
}

#[test]
fn anything_not_a_plain_dotted_quad_follows_the_chosen_network() {
    for ip in
        ["", "10", "10.0.2", "10.0.2.2.5", "10.0.2.256", "10.0.2.x", "kali.download", " 10.0.2.2"]
    {
        assert!(!is_local(ip), "{ip:?}");
    }
}
