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

//! The good handshake end to end: the hellos, the host accepted, then and
//! only then `USER_AUTH`, which the server accepts, then `AUTH_OK` and
//! session keys that work both ways.

use crate::kit::pair::Pair;
use crate::kit::proof::ProofSpec;
use crate::kit::server_auth::SESSION_ID;
use nsh_client::{Event, LoggedIn};
use nsh_wire::Kind;

#[test]
fn the_good_run_logs_in() {
    let mut p = Pair::good();
    let events = p.login();
    let caps = vec![b"CoreExec".to_vec(), b"IPC".to_vec()];
    let logged = LoggedIn { session_id: SESSION_ID, idle_s: 600, caps };
    assert_eq!(events, vec![Event::HostAccepted, Event::LoggedIn(logged)]);
    assert_eq!(p.client.attester().calls.get(), 1);
}

#[test]
fn nothing_follows_the_hello_until_the_host_proof_is_checked() {
    let mut p = Pair::good();
    let ch = p.client.take_output();
    assert_eq!((ch.len(), ch[4]), (4 + 1 + 1258, Kind::ClientHello.byte()));
    let sh = p.server.hello(&ch, |_| {});
    p.feed(&sh).unwrap();
    assert!(p.client.take_output().is_empty(), "no frame answers SERVER_HELLO");
    assert_eq!(p.client.attester().calls.get(), 0);
    let hp = p.proof(&ProofSpec::good());
    let (first, rest) = hp.split_at(hp.len() - 1);
    p.feed(first).unwrap();
    assert!(p.client.take_output().is_empty(), "a part of HOST_PROOF sends nothing");
    p.feed(rest).unwrap();
    let sent = p.sent();
    assert_eq!(sent.len(), 1);
    assert_eq!(sent[0].0, Kind::UserAuth.byte());
    assert!(p.server.user_auth(&sent[0].1));
}

#[test]
fn bytes_fed_one_at_a_time_give_the_same_login() {
    let mut p = Pair::good();
    p.to_proof();
    let hp = p.proof(&ProofSpec::good());
    hp.iter().for_each(|b| p.feed(&[*b]).unwrap());
    let sent = p.sent();
    assert!(p.server.user_auth(&sent[0].1));
}
