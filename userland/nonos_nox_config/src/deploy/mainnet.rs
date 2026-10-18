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

// NOX is the live V3_1 token behind this proxy: fees the Safe can change,
// skipped when either side is fee exempt, a blacklist, permit, and a revert
// on a zero-amount transfer. Count what arrives, never what was sent, and
// never send zero. Its block is V3_1's, the first the OS reads. The
// marketplace set is not deployed, and Shield L1 is not on mainnet (O3).
pub const MAINNET: Deployment = Deployment {
    nox_token: at(hex20("0a26c80be4e060e688d7c23addb92cbb5d2c9eca"), 25_960_151),
    settlement: None,
    escrow: None,
    publisher_bond: None,
    listing_registry: None,
    proof_anchor: None,
    shield_pool: None,
    amount_policy: None,
};
