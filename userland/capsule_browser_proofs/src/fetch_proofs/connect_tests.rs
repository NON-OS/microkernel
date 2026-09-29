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

//! A connect is started, polled, and answered in the call it completes.

use super::fetch_fixtures::{step, url_of};
use super::fetch_wire::FakeWire;
use crate::browser::fetch::open::open;
use crate::browser::fetch::progress::status;
use crate::browser::fetch::run::run;
use crate::browser::fetch::types::Phase;

#[test]
fn https_sends_its_client_hello_in_the_call_the_connect_completes() {
    let mut w = FakeWire::at(0);
    let mut f = open(&mut w, url_of("https://10.0.2.2:8443/"), None).expect("open");
    assert_eq!(status(&f), "Connecting to 10.0.2.2", "shown before anything is waited on");
    run(&mut w, &mut f, 30);
    assert_eq!(w.connects, vec![(f.handle, [10, 0, 2, 2], 8443)]);
    assert_eq!((f.phase, w.sent.len()), (Phase::Connecting, 0), "polled 0: still connecting");
    w.writable = true;
    run(&mut w, &mut f, 30);
    assert_eq!(f.phase, Phase::TlsFlight, "the hello went and the flight is awaited");
    assert_eq!(&w.sent_on(f.handle)[..3], &[22, 3, 1], "a ClientHello record");
    assert_eq!(w.connects.len(), 1, "the connect was started once");
}

#[test]
fn http_sends_its_request_in_that_call() {
    let mut w = FakeWire::at(0);
    let mut f = open(&mut w, url_of("http://10.0.2.2:8000/site.css"), None).expect("open");
    run(&mut w, &mut f, 30);
    w.writable = true;
    run(&mut w, &mut f, 30);
    assert_eq!(f.phase, Phase::ReadBody);
    let sent = w.sent_on(f.handle);
    assert!(sent.starts_with(b"GET /site.css HTTP/1.1\r\n"));
    assert!(sent.windows(22).any(|l| l == b"Connection: keep-alive"));
}

#[test]
fn a_connect_never_accepted_fails_at_eight_seconds() {
    let mut w = FakeWire::at(0);
    let mut f = open(&mut w, url_of("http://10.0.2.99/"), None).expect("open");
    run(&mut w, &mut f, 30);
    w.advance(8_000);
    step(&mut w, &mut f);
    assert_eq!(f.phase, Phase::Connecting, "eight seconds are allowed");
    w.advance(1);
    step(&mut w, &mut f);
    assert_eq!(f.error, Some("connect failed"));
}
