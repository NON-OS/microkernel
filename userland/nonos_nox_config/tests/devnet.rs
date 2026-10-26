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

//! The local test chain (P5) exists only in a `devnet` build: anywhere else
//! 31337 is an unknown chain, and nothing signs for it.

use nonos_nox_config::{address, Chain, ConfigError};

const LOCAL: u64 = 31_337;

#[cfg(not(feature = "devnet"))]
#[test]
fn a_release_build_knows_no_local_chain() {
    assert_eq!(Chain::from_id(LOCAL), None);
    assert_eq!(
        address(LOCAL, nonos_nox_config::Contract::NoxToken),
        Err(ConfigError::UnknownChain(LOCAL))
    );
}

#[cfg(feature = "devnet")]
#[test]
fn a_devnet_build_knows_it_and_nothing_is_deployed_there_yet() {
    assert_eq!(nonos_nox_config::DEVNET, LOCAL);
    assert_eq!(Chain::from_id(LOCAL), Some(Chain::Devnet));
    assert_eq!(Chain::Devnet.id(), LOCAL);
    assert_eq!(
        Chain::Devnet.block_ms(),
        nonos_nox_config::SEPOLIA_BLOCK_MS,
        "a local node mines at 12 s here too"
    );
    for c in nonos_nox_config::CONTRACTS {
        assert_eq!(address(LOCAL, c), Err(ConfigError::NotDeployed(c)), "{c:?} is not deployed");
    }
}
