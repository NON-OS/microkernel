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

//! A validly signed record of another session, another handshake, another
//! user or another document. The client refuses it when it arrives, and
//! the offline checker refuses it when told what the record must name.

use super::aborted_with;
use super::check::{gets, logged_in};
use super::own::{session_record, Other};
use super::record_good::report;
use crate::kit::server_auth::SESSION_ID;
use crate::kit::{attest::TestAttester, keys, pair::policy, record::SESSION};
use nsh_client::{Bound, Event, Mismatch, RecordRefusal, Refusal};
use nsh_wire::{Kind, Reason};

fn arrives(other: Other) -> Result<(), Refusal> {
    let mut p = logged_in();
    let th = p.client.th_server_hello().expect("the hello is in");
    assert_eq!((th, p.client.session_id()), (p.server.th_hello, Some(SESSION_ID)));
    let rec = session_record(th, other);
    let r = gets(&mut p, Kind::Record, &[&[1][..], &rec].concat());
    match r {
        Ok(()) => assert_eq!(p.client.events(), vec![Event::Record(rec)]),
        Err(_) => aborted_with(&mut p, Reason::Protocol),
    }
    r
}

#[test]
fn the_client_takes_only_this_sessions_record() {
    assert_eq!(arrives(Other::Nothing), Ok(()));
    let refused = |m| Err(Refusal::Record(m));
    assert_eq!(arrives(Other::Session), refused(Mismatch::SessionId));
    assert_eq!(arrives(Other::Transcript), refused(Mismatch::Transcript));
    assert_eq!(arrives(Other::Subject), refused(Mismatch::Subject));
    assert_eq!(arrives(Other::Document), refused(Mismatch::Document));
}

#[test]
fn offline_a_record_is_held_to_what_it_must_name() {
    let th = [0x7A; 64];
    let a = TestAttester::default();
    let check = |rec: &[u8], b: Bound| report(rec, &[], &b, &policy(), &a).map(|r| r.session_id);
    let good = session_record(th, Other::Nothing);
    let all = Bound {
        session_id: Some(SESSION),
        transcript: Some(th),
        subject: Some(keys::user().digest()),
    };
    assert_eq!(check(&good, Bound::default()), Ok(SESSION));
    assert_eq!(check(&good, all), Ok(SESSION));
    let unbound = |m| Err(RecordRefusal::Unbound(m));
    let other = session_record(th, Other::Session);
    assert_eq!(check(&other, Bound::default()).map(|s| s[0]), Ok(SESSION[0] ^ 1));
    assert_eq!(check(&other, all), unbound(Mismatch::SessionId));
    assert_eq!(check(&session_record(th, Other::Subject), all), unbound(Mismatch::Subject));
    let moved = session_record(th, Other::Transcript);
    assert_eq!(check(&moved, all), unbound(Mismatch::Transcript));
}
