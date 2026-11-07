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

//! `ESCALATE`: the role table checks it against the session's own login,
//! one at a time; a refusal is answered with its code and recorded, and
//! the session goes on; a granted escalation runs until its window closes,
//! its grant is revoked, or the session ends, and the record says which.

use super::good::{kinds, received};
use crate::kit::roles_build::grant::GrantB;
use crate::kit::run::Run;
use nsh_client::Event as C;
use nsh_roles::{grant_hash, Refusal as R};
use nsh_server::Why;
use nsh_wire::{EntryBody, Grant, Reason};

pub(super) fn esc_hash(g: &GrantB) -> [u8; 64] {
    grant_hash(&Grant::parse(&g.bytes()).expect("grant"))
}

pub(super) fn escalated(run: &Run) -> Vec<(Reason, u64)> {
    run.c_events
        .iter()
        .filter_map(|e| match e {
            C::Escalated { reason, expires_ms } => Some((*reason, *expires_ms)),
            _ => None,
        })
        .collect()
}

pub(super) fn refused_with(g: GrantB, code: Reason, why: R) {
    let mut run = Run::logged_in();
    run.user(|c| c.escalate(&g.bytes())).expect("escalate");
    assert_eq!(escalated(&run), [(code, 0)]);
    assert_eq!(run.refused(), [Why::Escalation(why)]);
    run.user(|c| c.input(b"still here")).expect("the session goes on");
    run.host(|s, now, r| s.exit(0, now, r)).expect("exit");
    let rec = received(&run);
    assert_eq!(kinds(&rec)[0], EntryBody::EscRefused { reason: code, grant: &esc_hash(&g) });
}
