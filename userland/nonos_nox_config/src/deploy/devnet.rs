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

//! The local test chain's contracts (P5, development images only). None is
//! deployed there yet: the test contracts wait on the owner (P1), so every
//! lookup but the chain itself answers "not deployed".

use super::Deployment;

pub const DEVNET: Deployment = Deployment {
    nox_token: None,
    settlement: None,
    escrow: None,
    publisher_bond: None,
    listing_registry: None,
    proof_anchor: None,
    shield_pool: None,
    amount_policy: None,
};
