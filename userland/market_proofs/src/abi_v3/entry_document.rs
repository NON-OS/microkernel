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

//! A listing served as a document of its own (plan/31, v1.4.16): the
//! wallet's publisher screen reads one entry, whole, to sign entries 2 and 3.

use nonos_marketplace_abi::{decode_entry_document, encode_entry_document, DecodeError};

use super::fixtures::sold;

#[test]
fn an_entry_document_reads_back_whole_and_canonical() {
    let mut entry = sold().entries[0].clone();
    entry.logo_hash = [0x3c; 32];
    let doc = encode_entry_document(&entry);
    let back = decode_entry_document(&doc).expect("decodes");
    assert_eq!(
        (back.listing_id.as_str(), back.publisher_pubkey, back.logo_hash),
        (entry.listing_id.as_str(), entry.publisher_pubkey, entry.logo_hash)
    );
    assert_eq!(encode_entry_document(&back), doc, "canonical");
    let mut tail = doc.clone();
    tail.push(0);
    assert!(matches!(decode_entry_document(&tail), Err(DecodeError::Trailing)));
    assert!(decode_entry_document(&doc[..doc.len() - 1]).is_err(), "cut");
    assert!(decode_entry_document(&[]).is_err(), "empty");
}
