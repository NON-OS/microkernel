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

use crate::chain::Chain;
use crate::contract::Contract;
use crate::deploy::deployment;
use crate::error::ConfigError;

/// The address of `contract` on `chain_id`, or why there is none.
pub const fn address(chain_id: u64, contract: Contract) -> Result<[u8; 20], ConfigError> {
    let chain = match Chain::from_id(chain_id) {
        Some(c) => c,
        None => return Err(ConfigError::UnknownChain(chain_id)),
    };
    match deployment(chain).get(contract) {
        Some(a) => Ok(a),
        None => Err(ConfigError::NotDeployed(contract)),
    }
}
