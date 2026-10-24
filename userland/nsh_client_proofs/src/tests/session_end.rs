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

//! How a session ends: `EXIT`, then the record in parts, then `CLOSE`, in
//! that order and no other. The record the client joins is the one the
//! host signed, and it checks offline against this handshake's document.

use super::check::{gets, logged_in};
use crate::kit::attest::TestAttester;
use crate::kit::enrol::EnrolSpec;
use crate::kit::pair::policy;
use crate::kit::record::{header, record_doc, Rec};
use nsh_client::{verify_record, Bound, Event, Refusal, Verdict};
use nsh_wire::Kind;

fn part(last: bool, bytes: &[u8]) -> Vec<u8> {
    [&[u8::from(last)][..], bytes].concat()
}

#[test]
fn exit_record_close_is_the_ending_and_the_record_checks() {
    let mut p = logged_in();
    let th = p.server.th_hello;
    let rec = Rec::for_handshake(th, header(&th, &EnrolSpec::good().bytes(), &record_doc(&th)), 70);
    let bytes = rec.bytes();
    gets(&mut p, Kind::Exit, &0i32.to_le_bytes()).unwrap();
    assert_eq!(p.client.input(b"x"), Err(Refusal::NotNow));
    let parts: Vec<&[u8]> = bytes.chunks(16_383).collect();
    for (i, c) in parts.iter().enumerate() {
        gets(&mut p, Kind::Record, &part(i + 1 == parts.len(), c)).unwrap();
    }
    gets(&mut p, Kind::Close, &[]).unwrap();
    let events = p.client.events();
    assert_eq!(events, vec![Event::Exit(0), Event::Record(bytes.clone()), Event::Closed]);
    assert!(p.client.is_over());
    assert_eq!(p.feed(&[0]), Err(Refusal::Ended));
    let (doc, a) = (record_doc(&th), TestAttester::default());
    let v = verify_record(&bytes, &doc, &[], &Bound::default(), &policy(), &a).map(|r| r.verdict);
    assert!(matches!(v, Ok(Verdict::Complete { entries: 71, .. })));
}
