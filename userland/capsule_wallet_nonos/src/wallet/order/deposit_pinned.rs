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

/*
 * What the config pins on a chain for deposit_check: the escrow, and the
 * forwarder's initcode hash its CREATE2 deposit addresses use.
 */

use nonos_nox_config::{address, forwarder_initcode_hash, Contract};

/// The escrow and the forwarder's initcode hash the config pins on a chain.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Pinned {
    pub escrow: [u8; 20],
    pub forwarder: [u8; 32],
}

/// What the config pins on `chain_id`, None until the owner deploys.
pub fn pinned(chain_id: u64) -> Option<Pinned> {
    let escrow = address(chain_id, Contract::Escrow).ok()?;
    Some(Pinned { escrow, forwarder: forwarder_initcode_hash(chain_id)? })
}
