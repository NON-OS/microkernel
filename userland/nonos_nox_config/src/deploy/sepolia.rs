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
// shield_core's ACTIVE pool (pools.rs PRODUCTION), from its deploy block.
pub const SEPOLIA: Deployment = Deployment {
    nox_token: at(hex20("3e5249a65ca513d5e11260222e0d26f46b465d36"), 0),
    settlement: None,
    escrow: None,
    publisher_bond: None,
    listing_registry: None,
    proof_anchor: None,
    shield_pool: at(hex20("aee51e82965ec1ded870f3f4c248ad4addc3e1cb"), 11_817_433),
};
