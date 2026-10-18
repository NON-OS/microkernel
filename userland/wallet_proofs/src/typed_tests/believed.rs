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

use nonos_marketplace_abi::Keccak;
use nonos_nox_config::EIP712_DOMAIN_TYPEHASH;
use sha3::{Digest, Keccak256};

use crate::typed::check::digest;
use crate::typed::{believed, Asked};

struct K;
impl Keccak for K {
    fn keccak256(&self, d: &[u8]) -> [u8; 32] {
        Keccak256::digest(d).into()
    }
}

const BOND: [u8; 20] = [0xB0; 20];
const ASKED: Asked =
    Asked { signer: [1; 20], struct_hash: [2; 32], chain_id: 11155111, bond: BOND };

/// The separator built by hand: the domain type, "NONOS Bond", "1", the
/// chain and the bond, each a word.
fn separator(chain: u64, bond: [u8; 20]) -> [u8; 32] {
    let mut c = [0u8; 32];
    c[24..].copy_from_slice(&chain.to_be_bytes());
    let mut b = [0u8; 32];
    b[12..].copy_from_slice(&bond);
    let name = K.keccak256(b"NONOS Bond");
    let version = K.keccak256(b"1");
    K.keccak256(&[EIP712_DOMAIN_TYPEHASH, name, version, c, b].concat())
}

fn reply(ds: [u8; 32], v: u8) -> Vec<u8> {
    let mut sig = [0x5a; 65];
    sig[64] = v;
    [&[1; 20][..], &ds, &[2; 32], &digest(&K, &ds, &[2; 32]), &sig].concat()
}

#[test]
fn believed_only_for_what_was_asked_under_the_bond_domain() {
    let ds = separator(11155111, BOND);
    let good = reply(ds, 27);
    assert_eq!(believed(&K, &good, &ASKED).map(|s| s[64]), Some(27));
    assert!(believed(&K, &reply(ds, 28), &ASKED).is_some());
    assert_eq!(believed(&K, &reply(ds, 29), &ASKED), None, "v");
    assert_eq!(believed(&K, &reply(separator(1, BOND), 27), &ASKED), None, "another chain");
    assert_eq!(
        believed(&K, &reply(separator(11155111, [0xB1; 20]), 27), &ASKED),
        None,
        "another bond"
    );
    for (at, why) in [(0, "signer"), (20, "separator"), (52, "structHash"), (84, "digest")] {
        let mut bad = good.clone();
        bad[at] ^= 1;
        assert_eq!(believed(&K, &bad, &ASKED), None, "{why}");
    }
    assert_eq!(believed(&K, &good[..180], &ASKED), None);
}
