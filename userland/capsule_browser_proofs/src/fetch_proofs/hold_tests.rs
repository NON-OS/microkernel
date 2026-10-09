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

//! A page asked of a network still connecting waits for it, says how far it
//! has got, and gives up with a reason after a bound. The first page after
//! a cold boot used to fail at net.anon's first "not connected yet".

use super::fetch_fixtures::{step, url_of};
use super::fetch_wire::FakeWire;
use crate::browser::fetch::open::open;
use crate::browser::fetch::progress::{status_at, waiting, QUIET_MS};
use crate::browser::fetch::proxy_fault::{NOT_READY, PROXY_FULL};
use crate::browser::fetch::socks::hold::{GAP_MS, HOLD_MS};
use crate::browser::fetch::types::{Fetch, Phase, Wait};
use crate::browser::net::mixnet::status::{read, Progress, STATUS_ASK};
use crate::browser::net::mixnet::{still, Heard, Network};

const NOT_YET: [u8; 10] = [5, 3, 0, 1, 0, 0, 0, 0, 0, 0];
const OK: [u8; 10] = [5, 0, 0, 1, 0, 0, 0, 0, 0, 0];

/// An Anyone fetch of example.org taken as far as its CONNECT.
fn connecting(w: &mut FakeWire) -> Fetch {
    w.mixnet = true;
    w.anyone = true;
    let mut f = open(w, url_of("http://example.org/"), None).expect("open");
    step(w, &mut f);
    w.deliver(f.handle, &[5, 0]);
    step(w, &mut f);
    assert_eq!(f.phase, Phase::SocksConnect);
    f
}

/// The proxy answers the CONNECT with `reply` and closes, as both do.
fn answer(w: &mut FakeWire, f: &mut Fetch, reply: &[u8]) {
    w.deliver(f.handle, reply);
    w.finished.push(f.handle);
    step(w, f);
}

#[test]
fn not_connected_yet_is_waited_for_on_a_new_conversation() {
    let mut w = FakeWire::at(0);
    let mut f = connecting(&mut w);
    let first = f.handle;
    answer(&mut w, &mut f, &NOT_YET);
    assert_eq!(f.error, None, "not a failure");
    assert_eq!(f.phase, Phase::SocksHello);
    assert_ne!(f.handle, first, "a new conversation");
    assert!(w.closed.contains(&first));
    assert_eq!(f.hold.map(|h| h.why), Some(Wait::NotYet));
    let sent = w.sent.len();
    step(&mut w, &mut f);
    assert_eq!(w.sent.len(), sent, "nothing is asked before the gap");
    w.advance(GAP_MS);
    step(&mut w, &mut f);
    assert_eq!(f.phase, Phase::SocksMethod, "the greeting goes again");
    w.deliver(f.handle, &[5, 0]);
    step(&mut w, &mut f);
    w.deliver(f.handle, &OK);
    step(&mut w, &mut f);
    assert_eq!(f.error, None);
    assert!(matches!(f.phase, Phase::SendReq | Phase::ReadBody), "the network came up: {:?}", f.phase);
    assert!(f.hold.is_some(), "kept, so the line can say how long it waited");
}

#[test]
fn a_network_that_never_connects_is_given_up_with_a_reason() {
    let mut w = FakeWire::at(0);
    let mut f = connecting(&mut w);
    answer(&mut w, &mut f, &NOT_YET);
    let since = f.hold.map(|h| h.since);
    let mut rounds = 0;
    while f.error.is_none() {
        rounds += 1;
        assert!(rounds < 400, "the wait is bounded");
        w.advance(GAP_MS);
        step(&mut w, &mut f);
        if f.phase == Phase::SocksMethod {
            w.deliver(f.handle, &[5, 0]);
            step(&mut w, &mut f);
        }
        if f.phase == Phase::SocksConnect {
            answer(&mut w, &mut f, &NOT_YET);
        }
        assert_eq!(f.hold.map(|h| h.since).or(since), since, "the bound runs from the first refusal");
    }
    assert_eq!(f.error, Some(NOT_READY));
    assert!(w.now.get() >= HOLD_MS && w.now.get() <= HOLD_MS + 10 * GAP_MS);
}

#[test]
fn a_proxy_with_no_room_is_waited_for_too() {
    let mut w = FakeWire::at(0);
    w.mixnet = true;
    let mut f = open(&mut w, url_of("http://example.org/"), None).expect("open");
    step(&mut w, &mut f);
    w.deliver(f.handle, &[5, 0xFF]);
    w.finished.push(f.handle);
    step(&mut w, &mut f);
    assert_eq!(f.error, None);
    assert_eq!(f.hold.map(|h| h.why), Some(Wait::Full));
    w.advance(HOLD_MS);
    step(&mut w, &mut f);
    w.deliver(f.handle, &[5, 0xFF]);
    w.finished.push(f.handle);
    step(&mut w, &mut f);
    assert_eq!(f.error, Some(PROXY_FULL));
}

#[test]
fn a_proxy_the_reader_set_that_wants_a_password_is_still_refused() {
    let mut w = FakeWire::at(0);
    w.writable = true;
    w.names.push((String::from("proxy.lan"), [10, 0, 0, 2]));
    let mut f = open(&mut w, url_of("http://example.org/"), Some(("proxy.lan", 1080))).expect("open");
    for _ in 0..4 {
        step(&mut w, &mut f);
        w.advance(10);
    }
    assert_eq!(f.phase, Phase::SocksMethod);
    w.deliver(f.handle, &[5, 0xFF]);
    step(&mut w, &mut f);
    assert_eq!(f.error, Some("socks auth rejected"), "only a network's own proxy is waited for");
}

#[test]
fn a_status_answer_is_read_only_when_it_is_one() {
    assert_eq!(STATUS_ASK, 6);
    assert_eq!(read(&[3, 1, 0, 5, 5]), Some(Progress { ready: false, step: 5, steps: 5, silent: None }));
    assert_eq!(read(&[3, 1, 1, 2, 2]), Some(Progress { ready: true, step: 2, steps: 2, silent: None }));
    assert_eq!(read(&[0]), None, "an older net.socks5 answers the ask as a reset");
    assert_eq!(read(&[1]), None, "an older net.anon closes it");
    assert_eq!(read(&[3, 2, 0, 1, 1]), None, "another format");
    assert_eq!(read(&[3, 1, 0, 6, 5]), None, "a step past the last");
    assert_eq!(read(&[3, 1, 0, 0, 5]), None, "no step 0");
    assert_eq!(read(&[3, 1, 2, 1, 5]), None);
    assert_eq!(read(&[3, 1, 0, 1, 5, 0, 0]), None, "too long");
}

#[test]
fn the_status_line_says_how_far_the_network_has_got() {
    let building = Heard::Known(Progress { ready: false, step: 5, steps: 5, silent: None });
    assert_eq!(
        still(Network::Anyone, Some(building)).as_deref(),
        Some("Anyone is still building its circuit (step 5 of 5)")
    );
    let relays = Heard::Known(Progress { ready: false, step: 2, steps: 5, silent: None });
    assert_eq!(
        still(Network::Anyone, Some(relays)).as_deref(),
        Some("Anyone is still fetching the list of relays (step 2 of 5)")
    );
    let parked = Heard::Known(Progress { ready: false, step: 1, steps: 2, silent: None });
    assert_eq!(
        still(Network::Nym, Some(parked)).as_deref(),
        Some("Nym is still waiting for net.nym to start (step 1 of 2)")
    );
    assert!(still(Network::Nym, Some(Heard::Busy)).is_some_and(|l| l.contains("mixnet session")));
    let ready = Heard::Known(Progress { ready: true, step: 5, steps: 5, silent: None });
    assert_eq!(still(Network::Anyone, Some(ready)), None);
    assert_eq!(still(Network::Anyone, Some(Heard::Unknown)), None);
}

#[test]
fn a_waiting_page_shows_the_network_and_the_seconds() {
    let mut w = FakeWire::at(0);
    let mut f = connecting(&mut w);
    assert!(!waiting(&f, 10), "a handshake just begun says only where it connects");
    assert!(waiting(&f, QUIET_MS));
    answer(&mut w, &mut f, &NOT_YET);
    let at = w.now.get() + 40_000;
    let heard = Heard::Known(Progress { ready: false, step: 5, steps: 5, silent: None });
    assert_eq!(
        status_at(&f, at, Some(heard)),
        "Connecting to example.org: Anyone is still building its circuit (step 5 of 5), 40 s"
    );
    assert_eq!(
        status_at(&f, at, Some(Heard::Unknown)),
        "Connecting to example.org: waiting for the Anyone network to connect, 40 s"
    );
}
