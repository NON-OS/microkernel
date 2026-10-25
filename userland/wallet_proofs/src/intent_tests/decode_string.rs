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

/* Calldata read back: an Approve's spender named by `at`, and Publish's
 * string held to the one shape the encoder writes. */

use nonos_nox_config::Contract;
use nonos_settle_proto::MAX_URL;

use super::decode::built;
use super::fixture::{at, n};
use super::kinds::every;
use crate::intent::{decode, Publisher};

fn padded(uri: &[u8]) -> Vec<u8> {
    let mut b = uri.to_vec();
    b.resize(uri.len().div_ceil(32) * 32, 0);
    b
}

/// Publish's heads, then an offset word, a length word and a body.
fn publish(off: [u8; 32], len: [u8; 32], body: &[u8]) -> Option<Publisher> {
    let cd = built(&every()[5]);
    decode(6, &[&cd[..4 + 160], &off, &len, body].concat(), at)
}

#[test]
fn an_approve_names_one_of_the_three_spenders() {
    let mut cd = built(&every()[0]);
    cd[4 + 31] = Contract::NoxToken as u8;
    assert_eq!(decode(1, &cd, at), None, "the token is not a spender");
    let alike = |c| at(if c == Contract::Escrow { Contract::PublisherBond } else { c });
    assert_eq!(decode(1, &built(&every()[0]), alike), None, "two spenders at one address");
    assert_eq!(decode(1, &built(&every()[0]), |_| None), None, "no spender deployed");
}

#[test]
fn publishs_string_is_canonical() {
    let uri = b"https://example.org/r/qwen-small@1";
    assert_eq!(publish(n(192), n(34), &padded(uri)), Some(every()[5].clone()));
    for off in [160, 224, 0] {
        assert_eq!(publish(n(off), n(34), &padded(uri)), None, "offset {off}");
    }
    let mut high = n(34);
    high[0] = 1;
    assert_eq!(publish(n(192), high, &padded(uri)), None, "a length out of its word");
    for len in [33, 35, 64, 0] {
        assert_eq!(publish(n(192), n(len), &padded(uri)), None, "length {len}");
    }
    let mut dirty = padded(uri);
    *dirty.last_mut().unwrap() = 1;
    assert_eq!(publish(n(192), n(34), &dirty), None, "padding not zero");
    assert_eq!(publish(n(192), n(34), uri), None, "padding missing");
    let http = b"http://example.org/r/qwen-small@1";
    assert_eq!(publish(n(192), n(33), &padded(http)), None, "not https");
    let mut long = b"https://h.org/".to_vec();
    long.resize(MAX_URL, b'a');
    assert!(publish(n(192), n(MAX_URL as u128), &padded(&long)).is_some(), "MAX_URL is taken");
    long.push(b'a');
    assert_eq!(publish(n(192), n(MAX_URL as u128 + 1), &padded(&long)), None, "past MAX_URL");
}
