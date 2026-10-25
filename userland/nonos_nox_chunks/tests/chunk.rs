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

#[path = "common/k.rs"]
mod k;
#[path = "common/word.rs"]
mod word;

use k::{kec, K};
use nonos_nox_chunks::{chunk, chunk_count, leaf, CHUNK, CIPHER_TREE, PLAIN_TREE};
use word::word;

#[test]
fn chunks_are_whole_and_the_last_is_padded() {
    assert_eq!([0, 1, 1024, 1025, 4096].map(chunk_count), [0, 1, 1, 2, 4]);
    let content: Vec<u8> = (0..1500u32).map(|i| i as u8).collect();
    assert_eq!(chunk(&content, 0).unwrap()[..], content[..CHUNK]);
    let last = chunk(&content, 1).unwrap();
    assert_eq!(last[..476], content[CHUNK..]);
    assert!(last[476..].iter().all(|b| *b == 0));
    assert_eq!(chunk(&content, 2), None);
    assert_eq!(chunk(&[], 0), None);
    assert_eq!(chunk(&content, u64::MAX), None);
}

#[test]
fn a_leaf_is_abi_encode_of_tree_index_and_hash() {
    let h = [7u8; 32];
    let want = kec(&[&word(&[1]), &word(&9u64.to_be_bytes()), &h]);
    assert_eq!(leaf(&K, CIPHER_TREE, 9, &h), want);
    assert_ne!(leaf(&K, PLAIN_TREE, 9, &h), want, "the two trees never share a leaf");
}
