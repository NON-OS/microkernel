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

/* The Bond's unbonding and withdraw, and the registry's withdraw of a
 * release, word for word: each selector the keccak of its signature, so
 * two withdraws of different contracts are never mixed up. */

use sha3::{Digest, Keccak256};

use super::decode::built;
use crate::intent::Publisher;

fn sel(sig: &str) -> Vec<u8> {
    Keccak256::digest(sig.as_bytes())[..4].to_vec()
}

#[test]
fn the_rest_word_for_word() {
    use Publisher::*;
    let unbond = RequestUnbond { bond_id: [2; 32] };
    assert_eq!(built(&unbond), [sel("requestUnbond(bytes32)"), vec![2; 32]].concat());
    let withdraw = WithdrawBond { bond_id: [9; 32] };
    assert_eq!(built(&withdraw), [sel("withdraw(bytes32)"), vec![9; 32]].concat());
    let release = WithdrawRelease { listing_id: [7; 32], release_id: [5; 32] };
    let want = [sel("withdraw(bytes32,bytes32)"), vec![7; 32], vec![5; 32]];
    assert_eq!(built(&release), want.concat());
    assert_ne!(sel("withdraw(bytes32)"), sel("withdraw(bytes32,bytes32)"));
}
