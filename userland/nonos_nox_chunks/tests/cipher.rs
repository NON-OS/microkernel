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
use nonos_nox_chunks::{chunk, chunk_root, cipher_root, key_commit, keystream_word, plain_hashes};
use nonos_nox_chunks::{leaf, root, root_of_hashes, xor_chunk, CIPHER_TREE, PLAIN_TREE};
use word::word;

const KEY: [u8; 32] = [0x5a; 32];

#[test]
fn the_keystream_is_abi_encode_of_key_chunk_and_word() {
    let want = kec(&[&KEY, &word(&3u64.to_be_bytes()), &word(&31u32.to_be_bytes())]);
    assert_eq!(keystream_word(&K, &KEY, 3, 31), want);
    assert_eq!(key_commit(&K, &KEY), kec(&[&KEY]));
}

#[test]
fn a_cipher_chunk_is_the_plain_chunk_xor_its_32_words() {
    let plain = [0x33u8; 1024];
    let c = xor_chunk(&K, &KEY, 4, &plain);
    for j in 0..32 {
        let ks = keystream_word(&K, &KEY, 4, j as u32);
        let want: Vec<u8> = ks.iter().map(|b| b ^ 0x33).collect();
        assert_eq!(c[32 * j..32 * j + 32], want[..]);
    }
    assert_eq!(xor_chunk(&K, &KEY, 4, &c), plain, "the same call decrypts");
    assert_ne!(xor_chunk(&K, &KEY, 5, &plain), c, "each chunk has its own stream");
}

#[test]
fn both_roots_from_their_parts() {
    let content: Vec<u8> = (0..5000u32).map(|i| (i * 7) as u8).collect();
    let hashes = plain_hashes(&K, &content);
    assert_eq!(hashes.len(), 5);
    assert_eq!(hashes[4], kec(&[&chunk(&content, 4).unwrap()]), "hashed padded");
    let plain_leaves: Vec<[u8; 32]> =
        (0..5).map(|i| leaf(&K, PLAIN_TREE, i, &hashes[i as usize])).collect();
    assert_eq!(chunk_root(&K, &content), root(&K, &plain_leaves));
    assert_eq!(chunk_root(&K, &content), root_of_hashes(&K, PLAIN_TREE, &hashes));
    let cipher: Vec<[u8; 32]> =
        (0..5).map(|i| kec(&[&xor_chunk(&K, &KEY, i, &chunk(&content, i).unwrap())])).collect();
    assert_eq!(cipher_root(&K, &KEY, &content), root_of_hashes(&K, CIPHER_TREE, &cipher));
    assert_ne!(cipher_root(&K, &KEY, &content), cipher_root(&K, &[0x5b; 32], &content));
    assert_eq!(chunk_root(&K, &[]), None);
}

#[test]
fn padding_is_why_content_len_is_committed_too() {
    let short = vec![9u8; 1025];
    let mut long = short.clone();
    long.resize(2048, 0);
    assert_eq!(chunk_root(&K, &short), chunk_root(&K, &long));
}
