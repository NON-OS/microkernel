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

//! Which builds trust the throwaway root: a `devnet` build (the devnet lane)
//! takes the local RPC's leaf by both ways a chain ends at a root, and every
//! other build, a release's, refuses it as an unknown issuer.

use sha2::{Digest, Sha256};

use super::dev_root::{HOST, LEAF, NOW, ROOT};
use crate::chain_walk::verify_chain;
use crate::roots::{find_spki_by_subject, is_trusted_spki_hash};
use crate::tests::cert_message_vectors::message;

fn root_hash() -> [u8; 32] {
    Sha256::digest(crate::cert_spki::cert_spki(ROOT).expect("spki")).into()
}

fn issuer() -> &'static [u8] {
    crate::cert_issuer::cert_issuer(LEAF).expect("issuer")
}

#[cfg(not(feature = "devnet"))]
#[test]
fn a_release_build_does_not_know_the_dev_root() {
    use crate::cert_problem::{cert_problem, CertProblem};
    assert!(!is_trusted_spki_hash(&root_hash()));
    assert_eq!(find_spki_by_subject(issuer()), None);
    for chain in [&[LEAF][..], &[LEAF, ROOT][..]] {
        assert_eq!(verify_chain(&message(chain), HOST, NOW), Ok(false));
        assert_eq!(cert_problem(&message(chain), HOST, NOW), Some(CertProblem::UnknownIssuer));
    }
}

#[cfg(feature = "devnet")]
#[test]
fn a_devnet_build_takes_the_local_rpc_leaf_and_nothing_else() {
    assert!(is_trusted_spki_hash(&root_hash()));
    assert_eq!(find_spki_by_subject(issuer()), crate::cert_spki::cert_spki(ROOT));
    for chain in [&[LEAF][..], &[LEAF, ROOT][..]] {
        assert_eq!(verify_chain(&message(chain), HOST, NOW), Ok(true));
        assert_eq!(verify_chain(&message(chain), b"example.com", NOW), Ok(false));
        assert_eq!(verify_chain(&message(chain), HOST, 20460101000000), Ok(false));
    }
}
