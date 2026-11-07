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

use super::escalate::*;
use crate::kit::raw_run::RawRun;
use crate::kit::rng::dry;
use crate::kit::roles_build::grant::GrantB;
use crate::kit::run::Run;
use nsh_server::{CallError, EndReason, Event as S};
use nsh_wire::Reason;

#[test]
fn a_host_that_cannot_start_an_escalation_ends_the_session() {
    let mut run = Run::logged_in();
    run.answer = false;
    run.user(|c| c.escalate(&GrantB::escalation().bytes())).expect("escalate");
    assert_eq!(escalated(&run), []);
    run.host(|s, now, r| s.escalation_failed(now, r)).expect("failed");
    assert_eq!(run.c_err, Some(nsh_client::Refusal::Aborted(Reason::Busy)));
    assert!(run.s_events.contains(&S::Closed(EndReason::Host)));
    let mut r = RawRun::session();
    assert_eq!(r.server.escalation_begun(r.now, &mut dry), Err(CallError::NotNow));
    assert_eq!(r.server.escalation_failed(r.now, &mut dry), Err(CallError::NotNow));
}

#[test]
fn an_escalation_asked_while_the_shell_stops_is_not_started() {
    let mut run = Run::logged_in();
    run.now += 600_000;
    run.host(|s, now, r| s.tick(now, r));
    run.user(|c| c.escalate(&GrantB::escalation().bytes())).expect("escalate");
    assert!(!run.s_events.iter().any(|e| matches!(e, S::Escalate { .. })));
    assert_eq!(escalated(&run), []);
}
