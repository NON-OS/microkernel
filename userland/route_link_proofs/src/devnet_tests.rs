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

//! The local test chain's RPC name (P5) is matched whole: no other name, not
//! one that holds it, starts with it or ends with it, is dialled directly.

use crate::devnet::{dial_at, is_devnet_rpc, DEVNET_RPC_AT, DEVNET_RPC_HOST};

#[test]
fn only_the_exact_name_is_the_local_chain() {
    assert!(is_devnet_rpc(DEVNET_RPC_HOST));
    assert_eq!(dial_at(DEVNET_RPC_HOST), DEVNET_RPC_AT);
    let near = [
        "rpc.devnet.nonos.test.evil.example",
        "evil.rpc.devnet.nonos.test",
        "rpc.devnet.nonos.tes",
        "RPC.DEVNET.NONOS.TEST",
        "rpc.devnet.nonos.test.",
        "",
        "ethereum-sepolia-rpc.publicnode.com",
    ];
    for host in near {
        assert!(!is_devnet_rpc(host), "{host} is not the local chain");
        assert_eq!(dial_at(host), host, "{host} is dialled as named");
    }
}

#[test]
fn the_local_chain_is_a_private_address_under_dot_test() {
    assert!(DEVNET_RPC_HOST.ends_with(".test"), "a reserved name no public host has");
    let octets: Vec<u8> = DEVNET_RPC_AT.split('.').map(|o| o.parse().unwrap()).collect();
    assert_eq!(octets[0], 10, "10/8, private: the development host, never the internet");
}
