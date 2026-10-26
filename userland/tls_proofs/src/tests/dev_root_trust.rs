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

//! Which builds take the local RPC's leaf: a `devnet` build (the devnet
//! lane), by the leaf alone or with the root over it, and no other. Both
//! lanes run every test here. In neither do the general lookups know the
//! root: its key is public, so only the dev anchor, which checks the name,
//! the leaf's use and the link, may take it.

use sha2::{Digest, Sha256};

use super::dev_root::{HOST, LEAF, NOW, ROOT};
use crate::cert_problem::{cert_problem, CertProblem};
use crate::chain_walk::verify_chain;
use crate::roots::{find_spki_by_subject, is_trusted_spki_hash};
use crate::tests::cert_message_vectors::message;

const DEVNET: bool = cfg!(feature = "devnet");

#[test]
fn no_build_puts_the_dev_root_in_the_general_lookups() {
    let h: [u8; 32] = Sha256::digest(crate::cert_spki::cert_spki(ROOT).expect("spki")).into();
    assert!(!is_trusted_spki_hash(&h));
    let issuer = crate::cert_issuer::cert_issuer(LEAF).expect("issuer");
    assert_eq!(find_spki_by_subject(issuer), None);
}

#[test]
fn only_a_devnet_build_takes_the_local_rpc_leaf() {
    for chain in [&[LEAF][..], &[LEAF, ROOT][..]] {
        let body = message(chain);
        assert_eq!(verify_chain(&body, HOST, NOW), Ok(DEVNET));
        let problem = (!DEVNET).then_some(CertProblem::UnknownIssuer);
        assert_eq!(cert_problem(&body, HOST, NOW), problem);
    }
}

#[test]
fn the_local_rpc_leaf_holds_for_its_name_and_window_only() {
    for chain in [&[LEAF][..], &[LEAF, ROOT][..]] {
        assert_eq!(verify_chain(&message(chain), b"example.com", NOW), Ok(false));
        assert_eq!(verify_chain(&message(chain), HOST, 20460101000000), Ok(false));
    }
}
