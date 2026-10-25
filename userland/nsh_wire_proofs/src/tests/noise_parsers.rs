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

//! Every parser in nsh_wire, run over one input. A parse that succeeds is
//! held to what success must mean wherever that is cheap to check.

use nsh_wire::{abort, auth_refused, data, empty, exit_status, plain_abort, signal, window};
use nsh_wire::{AuthOk, ClientHello, Enrol, EntryBody, EscalateResult, Grant, HostProof};
use nsh_wire::{Record, RecordPart, ServerHello, UserAuth};

pub fn every_parser(b: &[u8]) {
    let _ = (ClientHello::parse(b), ServerHello::parse(b), AuthOk::parse(b));
    let _ = (Enrol::parse(b), HostProof::parse(b), nsh_wire::escalate(b));
    let _ = (data(b), window(b), signal(b), exit_status(b), empty(b));
    let _ = (abort(b), auth_refused(b), plain_abort(b), RecordPart::parse(b));
    if let Ok(r) = EscalateResult::parse(b) {
        assert_eq!(r.reason == nsh_wire::Reason::Ok, r.expires_ms != 0);
    }
    if let Ok(g) = Grant::parse(b) {
        assert!(b.starts_with(g.issued) && g.issued.starts_with(g.signed));
    }
    if let Ok(u) = UserAuth::parse(b) {
        assert_eq!(u.grant.kind, nsh_wire::GrantKind::Base);
    }
    if let Some((kind, body)) = b.split_first() {
        let _ = EntryBody::parse(*kind, body);
    }
    if let Ok(r) = Record::parse(b) {
        assert_eq!(r.entries().count() as u32, r.footer.count);
    }
    for e in super::stages::ALL {
        if let Ok((kind, _)) = nsh_wire::split(b, e) {
            assert!(e.admits(kind));
        }
    }
}
