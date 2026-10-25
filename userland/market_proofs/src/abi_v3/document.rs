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

//! A release served as a document of its own: settle's FETCH holds
//! it only when keccak256 of its v3 signing form, recomputed from the decoded
//! release, is the registry's releaseHash.

use nonos_marketplace_abi::{decode_release_document, encode_release_document};
use nonos_marketplace_abi::{release_hash, release_signing_bytes, DecodeError};

use super::chain_hosts::Host;
use super::fixtures::sold;

#[test]
fn a_release_document_reads_back_and_hashes_its_signing_form() {
    let rel = sold().entries[0].releases[0].clone();
    assert!(release_hash(&Host, &rel).is_some());
    let doc = encode_release_document(&rel).expect("a schema 3 release");
    let back = decode_release_document(&doc).expect("decodes");
    assert_eq!(encode_release_document(&back), Some(doc.clone()), "canonical");
    assert_eq!(release_signing_bytes(&back), release_signing_bytes(&rel));
    assert_eq!(release_hash(&Host, &back), release_hash(&Host, &rel));
    assert_ne!(doc, release_signing_bytes(&rel), "the document is not its signing form");
    let mut tail = doc.clone();
    tail.push(0);
    assert!(matches!(decode_release_document(&tail), Err(DecodeError::Trailing)));
    assert!(decode_release_document(&doc[..doc.len() - 1]).is_err(), "cut");
    let mut v2 = rel.clone();
    v2.ext = None;
    assert_eq!(encode_release_document(&v2), None, "no v3 signing form");
}
