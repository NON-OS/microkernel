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


//! Short names end to end: the list fetched over an onion stream from a DNS
//! service (here the simulated service stands in for one of the six, its
//! key trusted in their place), verified, and used to open a stream.

use std::vec::Vec;

use ed25519_dalek::SigningKey;

use super::names_doc::{header, sign, signer};
use super::world::{net, Faults, Net};

const PUBLISHED: &str = "2026-09-18 00:00:00";
const VALID: &str = "2026-10-18 00:00:00";

/// A network whose service is trusted as the DNS service.
fn dns_net() -> (Net, SigningKey) {
    let mut n = net(Faults::default(), |_| {});
    let key = SigningKey::from_bytes(&n.world.service.seed);
    assert_eq!(key.verifying_key().to_bytes(), n.world.service.identity);
    let id = n.world.service.identity;
    n.state.names.signers = std::vec![id];
    n.state.names.services = std::vec![id];
    (n, key)
}

fn http(doc: &[u8]) -> Vec<u8> {
    let mut out = b"HTTP/1.0 200 OK\r\nContent-Type: text/plain\r\n\r\n".to_vec();
    out.extend_from_slice(doc);
    out
}

#[test]
fn a_short_name_resolves_through_the_signed_list_and_connects() {
    let (mut n, key) = dns_net();
    let address = n.address();
    let doc = sign(&(header(PUBLISHED, VALID) + &std::format!("lander.anyone {address}\n")), &key);
    n.world.service_reply = http(&doc);

    assert_eq!(n.open(b"lander.anyone", 80), Err("names pending"), "no list yet: ask again");
    n.run(60);
    let log = n.log();
    assert!(log.iter().any(|l| l.contains("names list verified, names")), "{log:#?}");
    assert_eq!(n.world.service_requests, [b"GET /tld/anyone HTTP/1.0\r\n\r\n".to_vec()], "the fork's fetch path");

    let id = n.open(b"Lander.Anyone.", 80).expect("resolved, in any case and rooted");
    n.run(40);
    assert_eq!(n.stream(id).0, "open");
    let log = n.log();
    assert!(log.iter().any(|l| l.contains("onion short name resolved by the signed list")));
    for line in &log {
        assert!(!line.to_ascii_lowercase().contains("lander"), "a name on the log: {line}");
        assert!(!line.contains(&address[..20]), "an address on the log: {line}");
    }
}

#[test]
fn a_list_from_any_other_signer_is_refused_and_nothing_resolves() {
    let (mut n, _) = dns_net();
    let address = n.address();
    let doc = sign(&(header(PUBLISHED, VALID) + &std::format!("lander.anyone {address}\n")), &signer("mallory"));
    n.world.service_reply = http(&doc);
    assert_eq!(n.open(b"lander.anyone", 80), Err("names pending"));
    n.run(60);
    assert!(n.log().iter().any(|l| l.contains("names list refused: signer is not an Anyone DNS service")));
    assert!(n.state.names.list.is_none());
    assert_eq!(n.open(b"lander.anyone", 80), Err("names pending"));
}

#[test]
fn an_expired_list_is_refused() {
    let (mut n, key) = dns_net();
    let address = n.address();
    let doc = sign(&(header("2026-08-01 00:00:00", "2026-09-01 00:00:00") + &std::format!("lander.anyone {address}\n")), &key);
    n.world.service_reply = http(&doc);
    let _ = n.open(b"lander.anyone", 80);
    n.run(60);
    assert!(n.log().iter().any(|l| l.contains("names list refused: expired")));
    assert!(n.state.names.list.is_none());
}

#[test]
fn a_name_the_list_lacks_is_unknown() {
    let (mut n, key) = dns_net();
    let address = n.address();
    let doc = sign(&(header(PUBLISHED, VALID) + &std::format!("lander.anyone {address}\n")), &key);
    n.world.service_reply = http(&doc);
    let _ = n.open(b"lander.anyone", 80);
    n.run(60);
    assert_eq!(n.open(b"elsewhere.anyone", 80), Err("name unknown"));
}

#[test]
fn a_name_that_moves_within_the_boot_is_refused() {
    let (mut n, key) = dns_net();
    let address = n.address();
    let doc = sign(&(header(PUBLISHED, VALID) + &std::format!("lander.anyone {address}\n")), &key);
    n.world.service_reply = http(&doc);
    let _ = n.open(b"lander.anyone", 80);
    n.run(60);
    assert!(n.open(b"lander.anyone", 80).is_ok(), "resolved and pinned");
    // A newer, validly signed list points the name at another service.
    let other = super::names_doc::address_of(&signer("elsewhere").verifying_key().to_bytes());
    let moved = sign(&(header("2026-09-18 12:00:00", VALID) + &std::format!("lander.anyone {other}\n")), &key);
    let newer = crate::onion::names::verify(&moved, &n.state.names.signers, n.now).expect("a valid list");
    n.state.names.list = Some(newer);
    assert_eq!(n.open(b"lander.anyone", 80), Err("name changed"));
    assert!(n.log().iter().any(|l| l.contains("onion short name now names another service than earlier this boot, refused")));
}

#[test]
fn a_full_address_never_touches_the_names_layer() {
    let (mut n, _) = dns_net();
    let address = n.address();
    let id = n.open(address.as_bytes(), 80).unwrap();
    n.run(40);
    assert_eq!(n.stream(id).0, "open");
    assert!(n.world.service_requests.is_empty(), "no list was fetched");
    assert!(n.state.names.list.is_none());
}

#[test]
fn a_replayed_older_list_never_replaces_a_newer_one() {
    let (mut n, key) = dns_net();
    let address = n.address();
    // A newer list is held, now expired, so the next name asks for a fetch.
    let held = sign(&(header("2026-09-10 00:00:00", "2026-09-15 00:00:00") + &std::format!("lander.anyone {address}\n")), &key);
    let mut held = crate::onion::names::verify(&held, &n.state.names.signers, 1_789_400_000).expect("valid then");
    held.published = n.now - 60;
    n.state.names.list = Some(held);
    // The service answers with a validly signed list published before it.
    let older = sign(&(header(PUBLISHED, VALID) + &std::format!("lander.anyone {address}\n")), &key);
    n.world.service_reply = http(&older);
    assert_eq!(n.open(b"lander.anyone", 80), Err("names pending"));
    n.run(60);
    assert!(n.log().iter().any(|l| l.contains("names list refused: older than the one held")));
    assert_eq!(n.state.names.list.as_ref().map(|l| l.published), Some(n.now - 60), "the held list stays");
}
