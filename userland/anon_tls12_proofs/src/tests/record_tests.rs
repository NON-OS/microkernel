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


//! The record layer's AEAD framing (RFC 7905 2 and RFC 5288 3), each
//! direction's sequence number, and its refusals.

use crate::tls12::record::{take, Direction};
use crate::tls12::Tls12Error;

fn pair(suite: u16) -> (Direction, Direction) {
    let d = |seq| Direction { suite, key: [0x11; 32], iv: [0x22; 12], seq };
    (d(0), d(0))
}

#[test]
fn both_suites_round_trip_and_count() {
    for suite in [0xCCA8, 0xC030] {
        let (mut tx, mut rx) = pair(suite);
        for i in 0..3u8 {
            let mut record = tx.seal(23, &[i; 40]).unwrap();
            let (kind, body) = take(&mut record, false).unwrap().unwrap();
            assert_eq!(kind, 23);
            assert_eq!(rx.open(23, &body).unwrap(), [i; 40]);
        }
        assert_eq!((tx.seq, rx.seq), (3, 3));
    }
}

#[test]
fn aes_gcm_carries_the_sequence_number_as_its_explicit_nonce() {
    let (mut tx, _) = pair(0xC030);
    tx.seq = 0x0102_0304_0506_0708;
    let record = tx.seal(23, b"x").unwrap();
    assert_eq!(&record[5..13], &[1, 2, 3, 4, 5, 6, 7, 8]);
    assert_eq!(record.len(), 5 + 8 + 1 + 16);
    let (mut tx, _) = pair(0xCCA8);
    assert_eq!(tx.seal(23, b"x").unwrap().len(), 5 + 1 + 16, "ChaCha20 sends no nonce");
}

#[test]
fn a_record_opened_as_another_type_or_out_of_order_or_altered_is_refused() {
    for suite in [0xCCA8, 0xC030] {
        let (mut tx, rx) = pair(suite);
        let record = tx.seal(23, b"data").unwrap();
        let body = &record[5..];
        let mut wrong_type = Direction { seq: rx.seq, ..pair(suite).1 };
        assert_eq!(wrong_type.open(21, body).err(), Some(Tls12Error::Crypto), "type is in the additional data");
        let mut wrong_seq = Direction { seq: 1, ..pair(suite).1 };
        assert_eq!(wrong_seq.open(23, body).err(), Some(Tls12Error::Crypto), "and so is the sequence number");
        for at in 0..body.len() {
            let mut b = body.to_vec();
            b[at] ^= 1;
            let mut fresh = pair(suite).1;
            assert!(fresh.open(23, &b).is_err(), "suite {suite:04x}: flip at {at}");
            assert_eq!(fresh.seq, 0, "a refused record does not advance the count");
        }
    }
}

#[test]
fn bounds_are_held() {
    let (mut tx, mut rx) = pair(0xC030);
    assert_eq!(tx.seal(23, &std::vec![0; (1 << 14) + 1]).err(), Some(Tls12Error::TooLarge));
    assert_eq!(rx.open(23, &[0; 7]).err(), Some(Tls12Error::Malformed), "shorter than the explicit nonce");
    assert_eq!(rx.open(23, &[0; 8 + 15]).err(), Some(Tls12Error::Malformed), "shorter than the tag");
    tx.seq = u64::MAX;
    assert_eq!(tx.seal(23, b"x").err(), Some(Tls12Error::Crypto), "the sequence number never wraps");
    let mut buf = std::vec![23, 3, 3, 0x48, 0x01];
    assert_eq!(take(&mut buf, false).err(), Some(Tls12Error::Malformed), "longer than 2^14 + 2048");
    let mut buf = std::vec![23, 3, 1, 0, 1, 0];
    assert_eq!(take(&mut buf, false).err(), Some(Tls12Error::Malformed), "a TLS 1.0 record after the hello");
    let mut buf = std::vec![22, 3, 1, 0, 1, 0];
    assert!(take(&mut buf, true).unwrap().is_some(), "but allowed for the server's hello");
    let mut buf = std::vec![23, 3, 3, 0, 0];
    assert_eq!(take(&mut buf, false).err(), Some(Tls12Error::Malformed), "an empty record");
}
