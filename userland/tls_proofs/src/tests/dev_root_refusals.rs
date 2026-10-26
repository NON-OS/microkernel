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

//! What anyone can sign with the throwaway root's public key (the files
//! tools/devnet/tls/refusals.py writes), refused by every build. Nothing here
//! or in dev_root_path is behind a feature, so the default lane runs it all
//! as the devnet lane, where the dev anchor's own checks refuse, does.

use super::dev_root::{HOST, NOW, ROOT};
use crate::cert_problem::{cert_problem, CertProblem};
use crate::chain_walk::verify_chain;
use crate::tests::cert_message_vectors::message;

static OTHER: &[u8] = include_bytes!("../../../../tools/devnet/tls/other-leaf.der");
static CLIENT: &[u8] = include_bytes!("../../../../tools/devnet/tls/client-leaf.der");
static NOEKU: &[u8] = include_bytes!("../../../../tools/devnet/tls/noeku-leaf.der");
static CA_LEAF: &[u8] = include_bytes!("../../../../tools/devnet/tls/ca-leaf.der");
const OTHER_HOST: &[u8] = b"rpc.mainnet.example";

// Every leaf is a good one but for the one thing its test is about: named
// for its host, inside its window and truly signed by its issuer.
pub(super) fn sound(leaf: &[u8], issuer: &[u8], host: &[u8]) {
    let spki = crate::cert_spki::cert_spki(issuer).expect("spki");
    assert_eq!(crate::verify_link::verify_link(leaf, spki), Ok(true));
    assert!(crate::cert_valid_now::cert_valid_now(leaf, NOW));
    assert!(crate::cert_dns_match::matches(leaf, host));
}

pub(super) fn refused(chain: &[&[u8]], host: &[u8]) {
    assert_eq!(verify_chain(&message(chain), host, NOW), Ok(false), "{} certs", chain.len());
}

#[test]
fn a_leaf_for_another_name_is_refused_even_for_that_name() {
    sound(OTHER, ROOT, OTHER_HOST);
    for chain in [&[OTHER][..], &[OTHER, ROOT][..]] {
        refused(chain, OTHER_HOST);
        let problem = cert_problem(&message(chain), OTHER_HOST, NOW);
        assert_eq!(problem, Some(CertProblem::UnknownIssuer));
        refused(chain, HOST);
    }
}
#[test]
fn a_leaf_not_stated_for_a_tls_server_is_refused() {
    for leaf in [CLIENT, NOEKU] {
        sound(leaf, ROOT, HOST);
        refused(&[leaf], HOST);
        refused(&[leaf, ROOT], HOST);
    }
}
#[test]
fn a_leaf_that_is_a_ca_is_refused() {
    sound(CA_LEAF, ROOT, HOST);
    assert!(crate::cert_is_ca::cert_is_ca(CA_LEAF));
    refused(&[CA_LEAF], HOST);
    refused(&[CA_LEAF, ROOT], HOST);
}
