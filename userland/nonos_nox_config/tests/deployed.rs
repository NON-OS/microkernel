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

use nonos_nox_config::{address, deployed, ConfigError, Contract, CONTRACTS, MAINNET, SEPOLIA};

#[test]
fn every_contract_carries_its_block_and_the_pool_is_sepolia_only() {
    for chain in [MAINNET, SEPOLIA] {
        for c in CONTRACTS {
            match (deployed(chain, c), address(chain, c)) {
                (Ok(d), Ok(a)) => assert_eq!(d.address, a),
                (Err(e), Err(f)) => assert_eq!(e, f),
                other => panic!("{c:?} on {chain}: {other:?}"),
            }
        }
    }
    let nox = deployed(MAINNET, Contract::NoxToken).expect("mainnet NOX");
    assert_eq!(nox.block, 25_960_151, "V3_1's block");
    let pool = deployed(SEPOLIA, Contract::ShieldPool).expect("the Sepolia pool");
    assert_eq!(pool.address[..4], [0xae, 0xe5, 0x1e, 0x82]);
    assert_eq!(pool.block, 0, "unknown until the owner pins it from its creation receipt");
    let policy = deployed(SEPOLIA, Contract::AmountPolicy).expect("the pool's policy");
    assert_eq!(policy.address[..4], [0x66, 0x0f, 0x66, 0xab]);
    assert_eq!(
        address(MAINNET, Contract::AmountPolicy),
        Err(ConfigError::NotDeployed(Contract::AmountPolicy))
    );
    let none = address(MAINNET, Contract::ShieldPool);
    assert_eq!(none, Err(ConfigError::NotDeployed(Contract::ShieldPool)), "O3");
}
