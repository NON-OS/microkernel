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
use nonos_settle_proto::EINVAL;

const MAINNET_HOSTS: [&str; 3] =
    ["ethereum-rpc.publicnode.com", "mainnet.gateway.tenderly.co", "rpc.mevblocker.io"];
const SEPOLIA_HOSTS: [&str; 3] = [
    "ethereum-sepolia-rpc.publicnode.com",
    "sepolia.gateway.tenderly.co",
    "rpc.sepolia.ethpandaops.io",
];

/// One chain's hosts, the current one first, and whether it said its chain.
pub struct Slot {
    pub hosts: [&'static str; 3],
    pub checked: bool,
}

pub struct Slots {
    mainnet: Slot,
    sepolia: Slot,
}

impl Slots {
    pub fn new() -> Slots {
        let slot = |hosts| Slot { hosts, checked: false };
        Slots { mainnet: slot(MAINNET_HOSTS), sepolia: slot(SEPOLIA_HOSTS) }
    }

    /// The chain's slot; EINVAL for a chain settle does not read.
    pub fn of(&mut self, chain_id: u64) -> Result<&mut Slot, i32> {
        match chain_id {
            MAINNET => Ok(&mut self.mainnet),
            SEPOLIA => Ok(&mut self.sepolia),
            _ => Err(EINVAL),
        }
    }
}
