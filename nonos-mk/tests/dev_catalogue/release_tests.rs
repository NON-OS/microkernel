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

use nonos_marketplace_abi::decode_index;

use crate::bootstrap_trust::is_trusted;
use crate::ingest::{load_verified, IngestError};
use crate::scratch::Scratch;
use crate::signed::{cli, signed};
use crate::verify::CryptoVerifier;

/// The operator key this checkout tracks, read from the tree as the market
/// capsule reads it (bootstrap_trust/keys.rs).
const TRACKED: [u8; 32] = *include_bytes!("../../../.keys/marketplace_operator_ed25519.pub");

/// A fresh throwaway key, made by `keygen` as nonos-dev-image makes the copy's.
fn throwaway(dir: &Scratch) -> (String, [u8; 32]) {
    let seed = dir.path("seed");
    let out = cli(&["keygen", "--out", &seed]);
    let key = hex::decode(out.split_whitespace().last().expect("keygen prints the key"));
    (seed, key.expect("hex").try_into().expect("32 bytes"))
}

/// Why a release market refuses the blob; taking it fails the test.
fn refusal(blob: &[u8]) -> IngestError {
    match load_verified(blob, &CryptoVerifier, 0) {
        Ok(v) => panic!(
            "a release market took the development catalogue: {} listings, index signature {}, \
             {} release signatures",
            v.index.entries.len(),
            v.signature_verified,
            v.publisher_signature_verified.len()
        ),
        Err(why) => why,
    }
}

#[test]
fn a_release_refuses_the_catalogue_under_a_throwaway_key() {
    let dir = Scratch::new("release-throwaway");
    let (seed, key) = throwaway(&dir);
    assert!(is_trusted(&TRACKED), "a release trusts the tracked operator key");
    assert!(key != TRACKED && !is_trusted(&key));
    assert!(refusal(&signed(&dir, &seed, &key)) == IngestError::UntrustedOperator);
}

/// The same bytes naming the tracked key as their operator pass the trust
/// check by name, so the signature alone has to stop them.
#[test]
fn a_release_refuses_the_catalogue_relabelled_as_its_operator() {
    let dir = Scratch::new("release-relabelled");
    let (seed, key) = throwaway(&dir);
    let mut blob = signed(&dir, &seed, &key);
    // The operator key is the first copy of it: it precedes every listing.
    let at = blob.windows(32).position(|w| w == key).expect("the operator key");
    blob[at..at + 32].copy_from_slice(&TRACKED);
    let named = decode_index(&blob).expect("still decodes").index.operator_pubkey;
    assert!(named == TRACKED && is_trusted(&named));
    assert!(refusal(&blob) == IngestError::SignatureRefused);
}
