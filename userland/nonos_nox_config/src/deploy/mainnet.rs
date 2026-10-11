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

use super::Deployment;
use crate::hex::hex20;

// NOX is the live V2.1 token: fee-on-transfer, capped, blacklisting. Count
// what arrives, never what was sent. The marketplace set is not deployed.
pub const MAINNET: Deployment = Deployment {
    nox_token: Some(hex20("0a26c80be4e060e688d7c23addb92cbb5d2c9eca")),
    settlement: None,
    escrow: None,
    publisher_bond: None,
    listing_registry: None,
    proof_anchor: None,
};
