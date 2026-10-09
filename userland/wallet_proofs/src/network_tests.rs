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

//! The two networks as the wallet tells them apart: each signs under its
//! own chain id, names itself, and only Sepolia has the shield pool.

use crate::wallet::chain::{current, named, pick, MAINNET, SEPOLIA};

#[test]
fn each_network_has_its_own_chain_id_and_name() {
    assert_eq!(MAINNET.id, 1);
    assert_eq!(SEPOLIA.id, 11_155_111);
    assert_eq!(named(MAINNET.id), MAINNET.name);
    assert_eq!(named(SEPOLIA.id), SEPOLIA.name);
    assert_ne!(named(5), MAINNET.name);
    /* No host serves both: a host's chain id is the network's own. */
    assert!(MAINNET.rpcs.iter().all(|h| !SEPOLIA.rpcs.contains(h)));
    assert!(MAINNET.rpcs.len() > 1 && SEPOLIA.rpcs.len() > 1);
}

#[test]
fn only_sepolia_has_the_shield_and_only_mainnet_staking() {
    /* Read through the pick, as the screens read it. */
    let _held = crate::fetch_tests::hold();
    pick(false);
    assert!(!current().shield);
    pick(true);
    assert!(current().shield);
    assert!(MAINNET.staking.is_some());
    assert!(SEPOLIA.staking.is_none());
    assert_ne!(MAINNET.nox, SEPOLIA.nox);
    assert_ne!(MAINNET.usdc, SEPOLIA.usdc);
}
