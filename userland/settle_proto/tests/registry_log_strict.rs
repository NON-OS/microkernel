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

use nonos_settle_proto::{Listed, MAX_URL, PUBLISHED_TOPIC};

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
fn a_published_log_reads_strictly_or_not_at_all() {
    let good = data(b"https://h/r");
    let no =
        |t: &[[u8; 32]], d: &[u8], why: &str| assert!(Listed::from_log(t, d, 1).is_none(), "{why}");
    no(&TOPICS[..3], &good, "a topic short");
    let mut other = TOPICS;
    other[0][31] ^= 1;
    no(&other, &good, "another event");
    no(&TOPICS, &data(b""), "no uri");
    no(&TOPICS, &data(&[b'u'; MAX_URL + 1]), "a uri past MAX_URL");
    no(&TOPICS, &good[..good.len() - 1], "short padding");
    no(&TOPICS, &[good.as_slice(), &[0; 32]].concat(), "a trailing word");
    no(&TOPICS, &good[..64], "no length");
    let mut bad = good.clone();
    bad[63] = 96;
    no(&TOPICS, &bad, "an offset other than 64");
    let mut bad = good.clone();
    bad[70] = 1;
    no(&TOPICS, &bad, "a length past u64");
    let mut bad = good.clone();
    *bad.last_mut().unwrap() = 1;
    no(&TOPICS, &bad, "dirty padding");
}
