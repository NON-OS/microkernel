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

use super::deployment::at;
use super::Deployment;
use crate::hex::hex20;

// The test token the wallet already offers on Sepolia; its block is not
// recorded (0), so no log read of it is made. The Shield pool is
// shield_core's ACTIVE pool (pools.rs PRODUCTION). Its deploy block is 0
// until the owner pins it from the pool's creation receipt with its tx hash
// (v1.4.17): 11,817,433 was never verified, and a block too late would make
// a topic0 read miss early events. The AmountPolicy is the pool's own
// (L1-Shield docs/deployments.md), checked against pool.amountPolicy().
pub const SEPOLIA: Deployment = Deployment {
    nox_token: at(hex20("3e5249a65ca513d5e11260222e0d26f46b465d36"), 0),
    settlement: None,
    escrow: None,
    publisher_bond: None,
    listing_registry: None,
    proof_anchor: None,
    shield_pool: at(hex20("aee51e82965ec1ded870f3f4c248ad4addc3e1cb"), 0),
    amount_policy: at(hex20("660f66ab31ca9919d9e1770fedc88ff2dd29ce59"), 0),
};
