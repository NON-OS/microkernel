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

//! What the oracle holds each frame and record to.

use super::host::doc_for;
use super::raw_run::RawRun;
use nsh_client::{verify_record, Bound};
use nsh_server::Event;
use nsh_wire::{abort, auth_refused, EscalateResult, Kind};

/// A frame the client may receive at that stage, with a code it may carry.
pub(super) fn allowed(stage: u8, kind: u8, body: &[u8]) {
    match (stage, Kind::from_byte(kind)) {
        (_, Ok(Kind::Abort)) => assert!(abort(body).is_ok(), "ABORT {body:?}"),
        (1, Ok(Kind::AuthRefused)) => assert!(auth_refused(body).is_ok(), "{body:?}"),
        (2, Ok(Kind::EscalateResult)) => assert!(EscalateResult::parse(body).is_ok(), "{body:?}"),
        other => panic!("a frame the client may not receive: {other:?}"),
    }
}

pub(super) fn record_checks(r: &RawRun) {
    let rec: Vec<u8> = r
        .events
        .iter()
        .filter_map(|e| match e {
            Event::Record(b) => Some(b.clone()),
            _ => None,
        })
        .flatten()
        .collect();
    let th = r.server.th_server_hello().expect("a handshake");
    let got = verify_record(
        &rec,
        &doc_for(&th),
        &[],
        &Bound::default(),
        &super::user::policy(),
        &super::user::attester(),
    );
    assert!(got.is_ok(), "{got:?}");
}
