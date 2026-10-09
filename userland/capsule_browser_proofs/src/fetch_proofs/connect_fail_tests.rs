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

//! Names, connects that cannot proceed, and connects being let go of.

use super::fetch_fixtures::url_of;
use super::fetch_wire::FakeWire;
use crate::browser::fetch::open::open;
use crate::browser::fetch::pool::Pool;
use crate::browser::fetch::retryable_error::retryable_error;
use crate::browser::fetch::run::run;
use crate::browser::fetch::security_error::security_error;
use crate::browser::fetch::types::Phase;

#[test]
fn an_unknown_name_fails_as_dns_and_is_not_retried() {
    let mut w = FakeWire::at(0);
    let mut f = open(&mut w, url_of("http://nowhere.test/"), None).expect("open");
    run(&mut w, &mut f, 30);
    run(&mut w, &mut f, 30);
    assert_eq!(f.error, Some("dns failed"));
    assert!(!retryable_error("dns failed"), "the resolver answered, and would again");
    assert!(retryable_error("connect failed") && !security_error("connect failed"));
}

#[test]
fn a_name_is_resolved_only_after_the_window_has_shown_it() {
    let mut w = FakeWire::at(0);
    w.names.push((String::from("relay.test"), [10, 0, 2, 2]));
    let mut f = open(&mut w, url_of("http://relay.test/"), None).expect("open");
    run(&mut w, &mut f, 30);
    assert_eq!((w.resolves, w.connects.len()), (0, 0), "nothing waited on in the first call");
    run(&mut w, &mut f, 30);
    assert_eq!(w.resolves, 1);
    assert_eq!(w.connects, vec![(f.handle, [10, 0, 2, 2], 80)]);
}

#[test]
fn cancelling_closes_a_pending_connect() {
    let mut w = FakeWire::at(0);
    let mut pool = Pool::new();
    let handle = pool.start(&mut w, url_of("http://10.0.2.99/"), None).expect("start").handle;
    assert!(pool.step(&mut w, 30).is_empty(), "still connecting");
    assert_eq!(pool.live[0].phase, Phase::Connecting);
    pool.cancel_all(&mut w);
    assert_eq!(w.closed, vec![handle]);
    assert!(!pool.busy());
}

#[test]
fn over_the_mixnet_the_greeting_goes_at_once() {
    let mut w = FakeWire::at(0);
    w.mixnet = true;
    let mut f = open(&mut w, url_of("https://example.test/"), None).expect("open");
    run(&mut w, &mut f, 30);
    assert_eq!(w.sent_on(f.handle), vec![5, 1, 0], "no connect: the SOCKS greeting");
    assert_eq!((f.phase, w.connects.len()), (Phase::SocksMethod, 0));
    assert!(f.keep, "a stream of its own at the proxy, kept for the next request");
}

/// No DNS server reachable is said as the network, not the spelling, and is
/// not tried again at once: net.dns said no upstream answered it, or
/// net.sockets, asked to look the name up itself, said the same (E_NO_DNS).
#[test]
fn no_dns_server_is_said_as_the_network_and_not_retried() {
    use crate::browser::fetch::wire::{HostFail, Resolved};
    use crate::browser::fetch::words::words;
    use crate::browser::net::mixnet::Way;

    let mut w = FakeWire::at(0);
    w.resolver = Some(Resolved::NoDns);
    let mut f = open(&mut w, url_of("http://example.test/"), None).expect("open");
    run(&mut w, &mut f, 30);
    run(&mut w, &mut f, 30);
    assert_eq!(f.error, Some("dns unreachable"));
    assert!(w.waited.is_empty(), "net.sockets is not asked to look it up again");

    let mut w = FakeWire::at(0);
    w.resolver = Some(Resolved::Unavailable);
    w.host_fail = Some(HostFail::NoDns);
    let mut f = open(&mut w, url_of("http://example.test/"), None).expect("open");
    run(&mut w, &mut f, 30);
    run(&mut w, &mut f, 30);
    assert_eq!(f.error, Some("dns unreachable"));
    assert_eq!(w.waited.len(), 1);

    let mut w = FakeWire::at(0);
    w.resolver = Some(Resolved::Unavailable);
    w.host_fail = Some(HostFail::Refused);
    let mut f = open(&mut w, url_of("http://example.test/"), None).expect("open");
    run(&mut w, &mut f, 30);
    run(&mut w, &mut f, 30);
    assert_eq!(f.error, Some("connect failed"), "a refused connect is still a refused connect");

    assert!(!retryable_error("dns unreachable"));
    let said = words("dns unreachable", Way::Direct, "example.test");
    assert!(said.contains("no DNS server is reachable") && said.contains("example.test"), "{said}");
    assert!(!said.contains("spelling"), "{said}");
}
