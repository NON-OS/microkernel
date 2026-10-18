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

use nonos_nox_config::{address, deployment, domain_words, Chain, ConfigError, Contract};
use nonos_nox_config::{BOND, ESCROW, LISTING, MAINNET, SEPOLIA, SETTLEMENT};

const NEW_SET: [Contract; 5] = [
    Contract::Settlement,
    Contract::Escrow,
    Contract::PublisherBond,
    Contract::ListingRegistry,
    Contract::ProofAnchor,
];

#[test]
fn chains_round_trip_and_nothing_else_resolves() {
    for c in [Chain::Mainnet, Chain::Sepolia] {
        assert_eq!(Chain::from_id(c.id()), Some(c));
    }
    for id in [0, 5, 10, 8453, 17000] {
        assert_eq!(Chain::from_id(id), None);
        assert_eq!(address(id, Contract::NoxToken), Err(ConfigError::UnknownChain(id)));
    }
}

#[test]
fn nox_is_known_on_both_chains_and_differs() {
    let main = address(MAINNET, Contract::NoxToken).expect("mainnet NOX");
    let test = address(SEPOLIA, Contract::NoxToken).expect("sepolia NOX");
    assert_ne!(main, test);
    assert_eq!(main[..4], [0x0a, 0x26, 0xc8, 0x0b]);
}

#[test]
fn an_undeployed_contract_refuses_rather_than_reading_zero() {
    for chain in [MAINNET, SEPOLIA] {
        for c in NEW_SET {
            let found = deployment(Chain::from_id(chain).unwrap()).get(c);
            match address(chain, c) {
                Ok(a) => assert_eq!(Some(a), found),
                Err(e) => assert_eq!((e, found), (ConfigError::NotDeployed(c), None)),
            }
            if let Ok(a) = address(chain, c) {
                assert_ne!(a, [0u8; 20], "a zero address is never a deployment");
            }
        }
        for d in [SETTLEMENT, ESCROW, BOND, LISTING] {
            if address(chain, d.contract).is_err() {
                assert_eq!(domain_words(&d, chain), Err(ConfigError::NotDeployed(d.contract)));
            }
        }
    }
}
