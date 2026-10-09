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


//! A service that restricts its clients, end to end: reached with the
//! key it lists, refused without one or with another.

use std::string::String;

use super::world::{client_secret, net, Faults, Net};
use crate::manager::{forget_client_key, set_client_key, KeyError};

fn base32(bytes: &[u8]) -> String {
    let alphabet = b"abcdefghijklmnopqrstuvwxyz234567";
    let (mut acc, mut bits, mut out) = (0u64, 0u32, String::new());
    for b in bytes {
        acc = (acc << 8) | *b as u64;
        bits += 8;
        while bits >= 5 {
            bits -= 5;
            out.push(alphabet[((acc >> bits) & 31) as usize] as char);
        }
    }
    if bits > 0 {
        out.push(alphabet[((acc << (5 - bits)) & 31) as usize] as char);
    }
    out
}

fn line(n: &Net, secret: &[u8; 32]) -> String {
    std::format!("{}:descriptor:x25519:{}", &n.address()[..56], base32(secret))
}

const RESTRICTED: Faults = Faults { client_auth: true, bad_signature: false, bad_first_intro_cert: false, pow_effort: None };

#[test]
fn the_listed_key_reaches_the_service() {
    let mut n = net(RESTRICTED, |_| {});
    let key = line(&n, &client_secret());
    assert_eq!(set_client_key(&mut n.state, key.as_bytes(), 9), Ok(()));
    let id = n.open(n.address().as_bytes(), 80).unwrap();
    n.run(40);
    assert_eq!(n.stream(id).0, "open");
}

#[test]
fn without_a_key_or_with_another_it_is_refused_and_named() {
    let mut n = net(RESTRICTED, |_| {});
    let id = n.open(n.address().as_bytes(), 80).unwrap();
    n.run(80);
    assert_eq!(n.stream(id).0, "ended 2");
    assert!(n.log().iter().any(|l| l.contains("onion service needs client authorization")));

    let mut n = net(RESTRICTED, |_| {});
    let other = line(&n, &[0x77; 32]);
    assert_eq!(set_client_key(&mut n.state, other.as_bytes(), 9), Ok(()));
    let id = n.open(n.address().as_bytes(), 80).unwrap();
    n.run(80);
    assert_eq!(n.stream(id).0, "ended 2");
}

#[test]
fn a_key_belongs_to_the_caller_that_gave_it() {
    let mut n = net(RESTRICTED, |_| {});
    let key = line(&n, &client_secret());
    assert_eq!(set_client_key(&mut n.state, key.as_bytes(), 9), Ok(()));
    let other = line(&n, &[0x77; 32]);
    assert_eq!(set_client_key(&mut n.state, other.as_bytes(), 10), Err(KeyError::NotYours));
    let address = &n.address().as_bytes()[..56].to_vec();
    assert_eq!(forget_client_key(&mut n.state, address, 10), Err(KeyError::NotYours));
    assert_eq!(forget_client_key(&mut n.state, address, 9), Ok(()));
    let id = n.open(n.address().as_bytes(), 80).unwrap();
    n.run(80);
    assert_eq!(n.stream(id).0, "ended 2", "with the key taken back, refused again");
}

#[test]
fn a_malformed_key_line_is_refused_and_nothing_is_kept() {
    let mut n = net(RESTRICTED, |_| {});
    assert_eq!(set_client_key(&mut n.state, b"not a key", 9), Err(KeyError::Malformed));
    let short = std::format!("{}:descriptor:x25519:abc", &n.address()[..56]);
    assert_eq!(set_client_key(&mut n.state, short.as_bytes(), 9), Err(KeyError::Malformed));
    assert!(n.state.client_keys.is_empty());
}

/* The key never reaches the serial log in any form. */
#[test]
fn the_key_is_never_on_the_log() {
    let mut n = net(RESTRICTED, |_| {});
    let secret = client_secret();
    let key = line(&n, &secret);
    set_client_key(&mut n.state, key.as_bytes(), 9).unwrap();
    let id = n.open(n.address().as_bytes(), 80).unwrap();
    n.run(40);
    assert_eq!(n.stream(id).0, "open");
    let b32 = base32(&secret);
    let hex: String = secret.iter().map(|b| std::format!("{b:02x}")).collect();
    for l in n.log() {
        let lower = l.to_ascii_lowercase();
        assert!(!lower.contains(&b32) && !lower.contains(&hex), "the log carries the key: {l}");
    }
}
