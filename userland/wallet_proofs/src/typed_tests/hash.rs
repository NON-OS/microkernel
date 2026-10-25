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
use nonos_nox_config::{TypedData, PUBLISHER_BINDING, RELEASE_ATTEST};
use sha3::{Digest, Keccak256};

use super::layout::words;
use crate::typed::struct_hash;

struct K;
impl Keccak for K {
    fn keccak256(&self, d: &[u8]) -> [u8; 32] {
        Keccak256::digest(d).into()
    }
}

fn manual(t: &TypedData, w: &[[u8; 32]]) -> [u8; 32] {
    let mut h = Keccak256::new();
    h.update(Keccak256::digest(t.type_string.as_bytes()));
    w.iter().for_each(|x| h.update(x));
    h.finalize().into()
}

#[test]
fn struct_hash_matches_manual() {
    for (t, n) in [(&RELEASE_ATTEST, 6), (&PUBLISHER_BINDING, 3)] {
        assert_eq!(t.fields, n);
        let w = words(n);
        assert_eq!(struct_hash(&K, t, &w), Some(manual(t, &w)));
        assert_eq!(struct_hash(&K, t, &words(n - 1)), None);
        assert_eq!(struct_hash(&K, t, &words(n + 1)), None);
    }
}

#[test]
fn wallet_entries_never_use_the_permit_domain() {
    for t in [&PUBLISHER_BINDING, &RELEASE_ATTEST] {
        assert_ne!(t.domain.name, "NONOS");
        assert_eq!(t.domain.name, "NONOS Bond");
    }
}
