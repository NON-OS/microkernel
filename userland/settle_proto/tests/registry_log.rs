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

use nonos_settle_proto::{withdrawn, Listed, MAX_URL, PUBLISHED, PUBLISHED_TOPIC};
use nonos_settle_proto::{WITHDRAWN, WITHDRAWN_TOPIC};
use sha3::{Digest, Keccak256};

fn word(n: usize) -> [u8; 32] {
    let mut w = [0u8; 32];
    w[24..].copy_from_slice(&(n as u64).to_be_bytes());
    w
}

fn data(uri: &[u8]) -> Vec<u8> {
    let mut d = [[9u8; 32], word(64), word(uri.len())].concat();
    d.extend_from_slice(uri);
    d.resize(96 + uri.len().div_ceil(32) * 32, 0);
    d
}

const TOPICS: [[u8; 32]; 4] = [PUBLISHED_TOPIC, [1; 32], [2; 32], [3; 32]];

#[test]
fn topics_are_the_signatures_hashes() {
    assert_eq!(<[u8; 32]>::from(Keccak256::digest(PUBLISHED)), PUBLISHED_TOPIC);
    assert_eq!(<[u8; 32]>::from(Keccak256::digest(WITHDRAWN)), WITHDRAWN_TOPIC);
}

#[test]
fn a_published_log_reads_whole() {
    let uri = b"https://example.org/qwen-small@1.release";
    let l = Listed::from_log(&TOPICS, &data(uri), 77).unwrap();
    let want = Listed {
        listing_id: [1; 32],
        release_id: [2; 32],
        release_hash: [9; 32],
        bond_id: [3; 32],
        block: 77,
        uri: uri.to_vec(),
    };
    assert_eq!(l, want);
    let exact = [b'a'; 32];
    assert_eq!(Listed::from_log(&TOPICS, &data(&exact), 1).unwrap().uri, exact);
    assert!(Listed::from_log(&TOPICS, &data(&[b'u'; MAX_URL]), 1).is_some());
}

#[test]
fn a_withdrawn_log_names_its_release() {
    let t = [WITHDRAWN_TOPIC, [1; 32], [2; 32]];
    assert_eq!(withdrawn(&t, &[]), Some(([1; 32], [2; 32])));
    assert_eq!(withdrawn(&t, &[0]), None);
    assert_eq!(withdrawn(&t[..2], &[]), None);
    assert_eq!(withdrawn(&[PUBLISHED_TOPIC, [1; 32], [2; 32]], &[]), None);
}
