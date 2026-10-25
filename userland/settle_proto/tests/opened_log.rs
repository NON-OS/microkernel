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

use nonos_settle_proto::{Opened, CHANNEL_OPENED, CHANNEL_OPENED_TOPIC};
use sha3::{Digest, Keccak256};

fn word(tail: &[u8]) -> [u8; 32] {
    let mut w = [0u8; 32];
    w[32 - tail.len()..].copy_from_slice(tail);
    w
}

#[test]
fn a_channel_opened_log_reads_whole_or_not_at_all() {
    assert_eq!(<[u8; 32]>::from(Keccak256::digest(CHANNEL_OPENED)), CHANNEL_OPENED_TOPIC);
    let topics = [CHANNEL_OPENED_TOPIC, [1; 32], word(&[2; 20]), word(&[3; 20])];
    let data: Vec<u8> = [
        [4; 32],
        word(&500u128.to_be_bytes()),
        word(&9u64.to_be_bytes()),
        word(&7u64.to_be_bytes()),
        word(&[5; 20]),
    ]
    .concat();
    let o = Opened::from_log(&topics, &data).unwrap();
    let want = Opened {
        channel_id: [1; 32],
        payer: [2; 20],
        payee: [3; 20],
        listing_id: [4; 32],
        amount: 500,
        expiry: 9,
        open_nonce: 7,
        reclaim_to: [5; 20],
    };
    assert_eq!(o, want);
    assert_eq!(Opened::from_log(&topics[..3], &data), None, "a topic short");
    assert_eq!(Opened::from_log(&topics, &data[..128]), None, "a word short");
    let mut other = topics;
    other[0][0] ^= 1;
    assert_eq!(Opened::from_log(&other, &data), None, "another event");
    for (at, why) in [
        (32, "an amount past u128"),
        (64 + 23, "an expiry past u64"),
        (128 + 11, "a dirty address"),
    ] {
        let mut bad = data.clone();
        bad[at] = 1;
        assert_eq!(Opened::from_log(&topics, &bad), None, "{why}");
    }
    let mut payer = topics;
    payer[1 + 1][0] = 1;
    assert_eq!(Opened::from_log(&payer, &data), None, "a dirty payer topic");
}
