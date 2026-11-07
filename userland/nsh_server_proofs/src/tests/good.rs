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

//! The good run: the real client accepts the host on the real `NONOSATT`
//! checker, the role table admits the user, a command runs, and the record
//! the host signed checks offline.

pub use crate::kit::host::doc_for;
use crate::kit::roles_build::keys::{key, K};
use crate::kit::run::Run;
use crate::kit::user::{attester, policy};
use nsh_client::{verify_record, Bound, Event as C};

/// The document the host sent in this run's `HOST_PROOF`.
pub fn sent_doc(run: &Run) -> Vec<u8> {
    doc_for(&run.server.th_server_hello().expect("a hello was sent"))
}

/// The record a run's client received, checked offline.
pub fn check(
    run: &Run,
    record: &[u8],
    ckpts: &[&[u8]],
) -> Result<nsh_client::Report, nsh_client::RecordRefusal> {
    let bound = Bound {
        session_id: run.client.session_id(),
        transcript: run.client.th_server_hello(),
        subject: Some(key(K::User).subject()),
    };
    verify_record(record, &sent_doc(run), ckpts, &bound, &policy(), &attester())
}

/// A record checked offline with nothing bound but the document.
pub fn check_alone(
    th: &[u8; 64],
    record: &[u8],
    ckpts: &[&[u8]],
) -> Result<nsh_client::Report, nsh_client::RecordRefusal> {
    verify_record(record, &doc_for(th), ckpts, &Bound::default(), &policy(), &attester())
}

/// The kinds of a record's entries, in order.
pub fn kinds(record: &[u8]) -> Vec<nsh_wire::EntryBody<'_>> {
    nsh_wire::Record::parse(record).expect("the record parses").entries().map(|e| e.body).collect()
}

pub fn received(run: &Run) -> Vec<u8> {
    let rec = run.c_events.iter().find_map(|e| match e {
        C::Record(r) => Some(r.clone()),
        _ => None,
    });
    rec.expect("the client received the record")
}
