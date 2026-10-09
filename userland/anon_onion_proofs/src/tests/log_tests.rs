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


//! The serial log names no service. Any local caller can read it, and a
//! line naming the address would tell it which service this machine used.

use std::string::String;
use std::vec::Vec;

use base64::engine::general_purpose::STANDARD_NO_PAD;
use base64::Engine;

use super::world::{net, Faults, Net};

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| std::format!("{b:02x}")).collect()
}

/// Every form the service could be named in.
fn names(n: &Net) -> Vec<String> {
    let s = &n.world.service;
    let address = n.address();
    std::vec![
        address[..56].to_ascii_lowercase(),
        address[..20].to_ascii_lowercase(),
        hex(&s.identity),
        hex(&s.identity).to_ascii_uppercase(),
        hex(&s.blinded.public),
        STANDARD_NO_PAD.encode(s.identity),
        STANDARD_NO_PAD.encode(s.blinded.public),
    ]
}

fn assert_unnamed(n: &Net, log: &[String]) {
    assert!(!log.is_empty(), "the lookup wrote its steps");
    for line in log {
        let lower = line.to_ascii_lowercase();
        for name in names(n) {
            assert!(!lower.contains(&name.to_ascii_lowercase()), "the log names the service: {line}");
        }
    }
}

#[test]
fn a_connect_names_no_service_on_the_log() {
    let mut n = net(Faults::default(), |_| {});
    let id = n.open(n.address().as_bytes(), 80).unwrap();
    n.run(40);
    assert_eq!(n.stream(id).0, "open");
    let log = n.log();
    assert_unnamed(&n, &log);
}

#[test]
fn every_failure_names_no_service_on_the_log() {
    for faults in [
        Faults { bad_signature: true, ..Faults::default() },
        Faults { client_auth: true, ..Faults::default() },
    ] {
        let mut n = net(faults, |_| {});
        let id = n.open(n.address().as_bytes(), 80).unwrap();
        n.run(80);
        assert_eq!(n.stream(id).0, "ended 2");
        let log = n.log();
        assert_unnamed(&n, &log);
    }
    let mut n = net(Faults::default(), |_| {});
    n.world.bad_rendezvous_mac = true;
    let id = n.open(n.address().as_bytes(), 80).unwrap();
    n.run(80);
    assert_eq!(n.stream(id).0, "ended 1");
    let log = n.log();
    assert_unnamed(&n, &log);
}

#[test]
fn a_refused_address_names_nothing_either() {
    let mut n = net(Faults::default(), |_| {});
    let mut host = n.address().into_bytes();
    host[0] = if host[0] == b'a' { b'b' } else { b'a' };
    assert_eq!(n.open(&host, 80), Err("bad onion"));
    let bad = String::from_utf8(host[..56].to_vec()).unwrap();
    assert!(n.log().iter().all(|l| !l.to_ascii_lowercase().contains(&bad)));
}
