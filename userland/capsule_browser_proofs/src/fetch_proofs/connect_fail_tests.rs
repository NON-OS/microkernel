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
fn an_unknown_name_fails_as_dns_and_may_be_retried() {
    let mut w = FakeWire::at(0);
    let mut f = open(&mut w, url_of("http://nowhere.test/"), None).expect("open");
    run(&mut w, &mut f, 30);
    run(&mut w, &mut f, 30);
    assert_eq!(f.error, Some("dns failed"));
    assert!(retryable_error("dns failed") && !security_error("dns failed"));
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
    assert_eq!((f.phase, w.connects.len(), f.keep), (Phase::SocksMethod, 0, false));
}
