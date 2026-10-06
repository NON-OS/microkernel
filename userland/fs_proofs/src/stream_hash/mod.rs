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

/*
 * A model download cut by a reboot goes on from the mark the kernel saved
 * on the volume (src/fs/blockfs_volume/import_feed): the writer's state and
 * the SHA-256 in flight, put down as bytes and picked up again. These hold
 * the hash half of that against the sha2 crate's own digest: hashed with a
 * save and a load at any byte, not only a block's edge, the file hashes to
 * what it hashes to whole, so a resumed download is held to its pin exactly
 * as one that never stopped. The kernel's files, mounted as they ship.
 */

#[path = "../../../../src/fs/blockfs_volume/import_feed/hash.rs"]
mod hash;
#[path = "../../../../src/fs/blockfs_volume/import_feed/hash_save.rs"]
mod hash_save;

use hash::PinHash;
use hash_save::HASH_STATE_BYTES;
use sha2::{Digest, Sha256};

fn bytes(n: usize) -> Vec<u8> {
    (0..n).map(|i| (i.wrapping_mul(2_654_435_761) >> 13) as u8).collect()
}

/* Hash `data`, putting the hash down and picking it up at each of `cuts`. */
fn resumed(data: &[u8], cuts: &[usize]) -> [u8; 32] {
    let mut h = PinHash::new();
    let mut at = 0;
    for &cut in cuts {
        h.update(&data[at..cut]);
        let mut saved = Vec::new();
        h.save(&mut saved);
        assert_eq!(saved.len(), HASH_STATE_BYTES);
        h = PinHash::load(saved.as_slice().try_into().unwrap());
        assert_eq!(h.taken(), cut as u64);
        at = cut;
    }
    h.update(&data[at..]);
    h.finish()
}

#[test]
fn a_hash_put_down_anywhere_finishes_as_the_whole_file_does() {
    let data = bytes(5 * 1024 + 37);
    let whole: [u8; 32] = Sha256::digest(&data).into();
    assert_eq!(resumed(&data, &[]), whole);
    for cut in [1, 63, 64, 65, 127, 128, 1000, 4096, data.len() - 1] {
        assert_eq!(resumed(&data, &[cut]), whole, "cut at {cut}");
    }
    assert_eq!(resumed(&data, &[7, 64, 65, 3000, 3001]), whole);
}

#[test]
fn a_hash_of_nothing_and_of_exact_blocks_is_sha256() {
    for n in [0usize, 55, 56, 64, 119, 120, 128] {
        let data = bytes(n);
        let whole: [u8; 32] = Sha256::digest(&data).into();
        assert_eq!(resumed(&data, &[]), whole, "{n} bytes");
    }
}
