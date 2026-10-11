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

//! The RPC hosts of each chain, the wallet's own (capsule_wallet_nonos
//! chain.rs). One is asked at a time; a failure moves to the next.

use nonos_nox_config::{MAINNET, SEPOLIA};

const MAINNET_HOSTS: [&str; 3] =
    ["ethereum-rpc.publicnode.com", "mainnet.gateway.tenderly.co", "rpc.mevblocker.io"];
const SEPOLIA_HOSTS: [&str; 3] = [
    "ethereum-sepolia-rpc.publicnode.com",
    "sepolia.gateway.tenderly.co",
    "rpc.sepolia.ethpandaops.io",
];

/// The chain's slot in settle's tables, and its hosts.
pub fn hosts(chain_id: u64) -> Option<(usize, &'static [&'static str])> {
    match chain_id {
        MAINNET => Some((0, &MAINNET_HOSTS)),
        SEPOLIA => Some((1, &SEPOLIA_HOSTS)),
        _ => None,
    }
}
