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

//! A response read as it arrives opens each record once.

use alloc::vec::Vec;

use crate::app_reader::AppReader;
use crate::traffic_keys::TrafficKeys;

pub fn keys(suite: u16) -> TrafficKeys {
    TrafficKeys {
        suite,
        handshake_secret: [0; 32],
        client_secret: [1; 32],
        server_secret: [2; 32],
        client_key: [3; 32],
        client_iv: [4; 12],
        server_key: [5; 32],
        server_iv: [6; 12],
    }
}

/// `body` as server application records of `size` bytes, from `first`.
pub fn records(app: &TrafficKeys, body: &[u8], size: usize, first: u64) -> (Vec<u8>, usize) {
    let mut wire = Vec::new();
    let mut n = 0;
    for (i, chunk) in body.chunks(size).enumerate() {
        let (key, iv) = (&app.server_key, &app.server_iv);
        let seq = first + i as u64;
        let record = crate::record_seal::seal(app.suite, key, iv, seq, 23, chunk).expect("seal");
        wire.extend_from_slice(&record);
        n += 1;
    }
    (wire, n)
}

/// Feed `wire` to a fresh reader `step` bytes at a time, as reads bring it.
pub fn trickle(app: &TrafficKeys, wire: &[u8], step: usize) -> (AppReader, usize) {
    let (mut reader, mut opened, mut at) = (AppReader::new(), 0, 0);
    while at < wire.len() {
        at = (at + step).min(wire.len());
        opened += reader.feed(app, &wire[..at]);
    }
    (reader, opened)
}

#[test]
fn every_split_reads_what_one_pass_reads_and_opens_each_record_once() {
    let body: Vec<u8> = (0..1500u32).map(|i| (i * 7 + i / 13) as u8).collect();
    for suite in [0x1301, 0x1303] {
        let app = keys(suite);
        let (wire, n) = records(&app, &body, 330, 0);
        let whole = crate::application_plaintext::application_plaintext_cached(&app, &wire);
        assert_eq!(whole, body, "one pass");
        for step in 1..=wire.len() {
            let (reader, opened) = trickle(&app, &wire, step);
            assert_eq!(reader.plaintext(), &body[..], "suite {suite:#x} step {step}");
            assert_eq!(opened, n, "suite {suite:#x} step {step}");
        }
    }
}
