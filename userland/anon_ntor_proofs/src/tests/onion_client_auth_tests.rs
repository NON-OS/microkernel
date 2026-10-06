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
//! Join.
//! Client authorization, held to vectors/client_auth_vector.expect, written
//! by vectors/client_auth_vector.py from the fork's cookie derivation.

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

use crate::hex;
use crate::onion::client_auth::{auth_layer, cookie, parse_key};

const EXPECT: &str = include_str!("../../vectors/client_auth_vector.expect");
const ADDR: &str = "25njqamcweflpvkl73j4szahhihoc4xt3ktcgjnpaingr5yhkenctcid";

fn vector() -> BTreeMap<String, String> {
    EXPECT.lines().filter_map(|l| l.split_once(' ')).map(|(k, v)| (k.into(), v.trim().into())).collect()
}

fn middle(v: &BTreeMap<String, String>, ours_at: usize) -> String {
    let eph = base64_of(&hex(&v["ephemeral"]));
    let mut text = alloc::format!("desc-auth-type x25519\ndesc-auth-ephemeral-key {eph}\n");
    for i in 0..16u8 {
        if i as usize == ours_at {
            text += &alloc::format!("auth-client {}\n", v["line"]);
        } else {
            let id = base64_of(&[i; 8]);
            let iv = base64_of(&[i; 16]);
            text += &alloc::format!("auth-client {id} {iv} {iv}\n");
        }
    }
    text + "encrypted\n"
}

fn base64_of(bytes: &[u8]) -> String {
    String::from_utf8(crate::base64_encode::encode(bytes)).unwrap()
}

#[test]
fn the_cookie_comes_out_of_our_entry() {
    let v = vector();
    let layer = auth_layer(middle(&v, 5).as_bytes()).expect("an x25519 client list");
    assert_eq!(layer.ephemeral.to_vec(), hex(&v["ephemeral"]));
    assert_eq!(layer.clients.len(), 16);
    let seed: [u8; 32] = hex(&v["seed"]).try_into().unwrap();
    let got = cookie(&[0x63; 32], &seed, &layer).expect("our entry is found");
    assert_eq!(got.to_vec(), hex(&v["cookie"]));
}

#[test]
fn another_clients_seed_or_subcredential_finds_nothing() {
    let v = vector();
    let layer = auth_layer(middle(&v, 0).as_bytes()).unwrap();
    let seed: [u8; 32] = hex(&v["seed"]).try_into().unwrap();
    let mut other = seed;
    other[0] ^= 1;
    assert!(cookie(&[0x63; 32], &other, &layer).is_none());
    assert!(cookie(&[0x64; 32], &seed, &layer).is_none());
    let none = auth_layer(b"desc-auth-type x25519\ndesc-auth-ephemeral-key AAAA\n");
    assert!(none.is_none(), "an ephemeral key that is not 32 bytes");
    assert!(auth_layer(b"desc-auth-type ed25519\n").is_none());
}

#[test]
fn a_key_line_parses_in_the_forks_format() {
    let v = vector();
    let line = alloc::format!("{ADDR}:descriptor:x25519:{}", v["secret_b32"]);
    let key = parse_key(line.as_bytes()).expect("parses");
    assert_eq!(key.secret(), &[0x61; 32]);
    assert_eq!(key.identity.to_vec(), hex("d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a"));
    let suffixed = alloc::format!("{ADDR}.anyone:descriptor:x25519:{}\n", v["secret_b32"].to_uppercase());
    assert!(parse_key(suffixed.as_bytes()).is_some(), "with the suffix, upper case and a newline");
}

#[test]
fn key_lines_that_are_refused() {
    let v = vector();
    let s = &v["secret_b32"];
    let bad: Vec<String> = alloc::vec![
        alloc::format!("{ADDR}:descriptor:ed25519:{s}"),
        alloc::format!("{ADDR}:intro:x25519:{s}"),
        alloc::format!("{ADDR}:descriptor:x25519:{}", &s[1..]),
        alloc::format!("{ADDR}:descriptor:x25519:{}b", &s[..51]),
        alloc::format!("{ADDR}:descriptor:x25519:{}", "a".repeat(52)),
        alloc::format!("{}:descriptor:x25519:{s}", &ADDR[1..]),
        alloc::format!("{ADDR}:descriptor:x25519"),
        alloc::format!("{ADDR}:descriptor:x25519:{s}:extra"),
    ];
    for line in bad {
        assert!(parse_key(line.as_bytes()).is_none(), "{line}");
    }
}
