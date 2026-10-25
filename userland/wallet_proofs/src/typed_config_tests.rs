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
//! Every verifying contract the keyring signs under is nonos_nox_config's,
//! on every chain it signs for (D12), so a redeploy is a change there alone.
//! Where the config has no address the entry refuses, as it does today.

use nonos_nox_config::{deployment, Chain, Contract};

use super::typed_fixture::EVERY;
use super::typed_hash_tests::keccak;
use crate::server::typed::check::{admit, admit_in};
use crate::server::typed::hashes::hashes;
use crate::server::typed::refusal::Refusal;
use crate::server::typed::table::ENTRIES;

const CHAINS: [Chain; 2] = [Chain::Mainnet, Chain::Sepolia];

/// Each entry's verifying contract, written out so a table naming another fails.
fn verifier(id: u16) -> Contract {
    match id {
        1 => Contract::Settlement,
        2 | 3 => Contract::PublisherBond,
        _ => panic!("no entry {id}"),
    }
}

#[test]
fn the_verifying_contract_is_the_configs() {
    for e in ENTRIES {
        let words = vec![0u8; e.fields.len() * 32];
        for c in CHAINS {
            let want = deployment(c).get(verifier(e.id));
            assert_eq!(e.contract(c, deployment), want, "entry {} on {c:?}", e.id);
            let admitted = admit(e, c.id(), &words);
            assert_eq!(admitted, want.ok_or(Refusal::NotDeployed), "entry {} on {c:?}", e.id);
        }
    }
}

/// A deployed address is the one the domain separator and the digest are
/// taken over: moving it moves the signature.
#[test]
fn a_deployed_address_reaches_the_digest() {
    let k = |b: &[u8]| keccak(b).unwrap();
    for e in ENTRIES {
        let words = vec![0u8; e.fields.len() * 32];
        for c in CHAINS {
            let at = admit_in(e, c.id(), &words, |_| &EVERY).unwrap();
            assert_eq!(Some(at), EVERY.get(verifier(e.id)), "entry {} on {c:?}", e.id);
            let h = hashes(e, c.id(), &at, &words, keccak).unwrap();
            let mut dom = [0u8; 160];
            dom[..32].copy_from_slice(&k(nonos_nox_config::EIP712_DOMAIN_TYPE.as_bytes()));
            dom[32..64].copy_from_slice(&k(e.typed.domain.name.as_bytes()));
            dom[64..96].copy_from_slice(&k(b"1"));
            dom[120..128].copy_from_slice(&c.id().to_be_bytes());
            dom[140..].copy_from_slice(&at);
            assert_eq!(h.domain_separator, k(&dom), "entry {} on {c:?}", e.id);
            let moved = hashes(e, c.id(), &[0xEE; 20], &words, keccak).unwrap();
            assert_ne!(moved.digest, h.digest, "entry {} on {c:?}", e.id);
        }
    }
}
