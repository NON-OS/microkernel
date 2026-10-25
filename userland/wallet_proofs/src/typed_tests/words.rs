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

use nonos_marketplace_abi::{bond_id, Keccak};
use sha3::{Digest, Keccak256};

use crate::attest::Attest;
use crate::typed::{attest_words, binding_words, Wrong};

struct K;
impl Keccak for K {
    fn keccak256(&self, d: &[u8]) -> [u8; 32] {
        Keccak256::digest(d).into()
    }
}

const ME: [u8; 20] = [0x11; 20];
const KEY: [u8; 32] = [0x22; 32];

fn attest(chunk: u8, len: u64, pkg: u8) -> Attest {
    Attest {
        chain_id: 11155111,
        listing_id: [1; 32],
        release_id: [2; 32],
        bond_id: bond_id(&K, &ME, &KEY),
        chunk_root: [chunk; 32],
        content_len: len,
        package_hash: [pkg; 32],
    }
}

#[test]
fn a_binding_names_this_accounts_bond() {
    let bond = bond_id(&K, &ME, &KEY);
    assert_eq!(binding_words(&K, &ME, &KEY, &[1; 32], &bond), Ok([KEY, [1; 32], bond]));
    assert_eq!(binding_words(&K, &[0x12; 20], &KEY, &[1; 32], &bond), Err(Wrong::NotOurBond));
    assert_eq!(binding_words(&K, &ME, &[0x23; 32], &[1; 32], &bond), Err(Wrong::NotOurBond));
}

#[test]
fn an_attest_is_escrow_or_package_and_nothing_between() {
    let escrow = attest_words(&K, &ME, &KEY, &attest(7, 0x0102, 0)).unwrap();
    let mut len = [0u8; 32];
    len[30..].copy_from_slice(&[1, 2]);
    assert_eq!(escrow, [[1; 32], [2; 32], bond_id(&K, &ME, &KEY), [7; 32], len, [0; 32]]);
    let package = attest_words(&K, &ME, &KEY, &attest(0, 0, 9)).unwrap();
    assert_eq!((package[3], package[4], package[5]), ([0; 32], [0; 32], [9; 32]));
    for (c, l, p) in [(7, 5, 9), (0, 0, 0), (7, 0, 0), (0, 5, 0), (0, 5, 9), (7, 0, 9)] {
        assert_eq!(attest_words(&K, &ME, &KEY, &attest(c, l, p)), Err(Wrong::Shape), "{c} {l} {p}");
    }
    let mut other = attest(7, 5, 0);
    other.bond_id[0] ^= 1;
    assert_eq!(attest_words(&K, &ME, &KEY, &other), Err(Wrong::NotOurBond));
}
