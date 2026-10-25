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

//! The networks themselves: what each one is called and where its requests
//! and contracts go.

use super::chain_hex::{hex20, nox_of};

pub struct Chain {
    pub id: u64,
    /// As the status line and the review say it.
    pub name: &'static str,
    /// The JSON-RPC hosts, reached over TLS 1.3 on port 443, in the order
    /// they are tried. Each serves a batch of every call a refresh makes at
    /// `/` without a key. One that fails is left for the next, and the next
    /// is asked its chain id before anything else is read from it.
    pub rpcs: &'static [&'static str],
    pub nox: [u8; 20],
    pub usdc: [u8; 20],
    /// The NOX staking contract, on mainnet only.
    pub staking: Option<[u8; 20]>,
    /// Whether the NOX Shield pool runs on this network.
    pub shield: bool,
    pub explorer: &'static str,
}

pub const MAINNET: Chain = Chain {
    id: 1,
    name: "Ethereum mainnet",
    rpcs: &["ethereum-rpc.publicnode.com", "mainnet.gateway.tenderly.co", "rpc.mevblocker.io"],
    nox: nox_of(1),
    usdc: hex20("a0b86991c6218b36c1d19d4a2e9eb0ce3606eb48"),
    staking: Some(hex20("a94d6009790ba13597a1e1b7cf4e1531ea513613")),
    shield: false,
    explorer: "etherscan.io",
};

pub const SEPOLIA: Chain = Chain {
    id: 11_155_111,
    name: "Sepolia",
    rpcs: &[
        "ethereum-sepolia-rpc.publicnode.com",
        "sepolia.gateway.tenderly.co",
        "rpc.sepolia.ethpandaops.io",
    ],
    nox: nox_of(11_155_111),
    usdc: hex20("1c7d4b196cb0c7b01d743fbc6116a902379c7238"),
    staking: None,
    shield: true,
    explorer: "sepolia.etherscan.io",
};
