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
use super::good::{check, kinds, received};
use crate::kit::roles_build::grant::GrantB;
use crate::kit::roles_build::{HOUR, NOW};
use crate::kit::run::Run;
use nsh_client::Verdict;
use nsh_server::Event as S;
use nsh_wire::{EntryBody, Reason};

#[test]
fn an_escalation_runs_until_its_window_closes() {
    let mut run = Run::logged_in();
    let g = GrantB::escalation();
    run.user(|c| c.escalate(&g.bytes())).expect("escalate");
    let ends = NOW + HOUR / 2;
    assert!(run
        .s_events
        .iter()
        .any(|e| matches!(e, S::Escalate { ends_ms, .. } if *ends_ms == ends)));
    assert_eq!(escalated(&run), [(Reason::Ok, ends)]);
    run.now = ends - 1;
    run.user(|c| c.input(b"x")).expect("input");
    run.host(|s, now, r| s.tick(now, r));
    assert!(!run.s_events.contains(&S::EscalationEnded));
    run.now = ends;
    run.host(|s, now, r| s.tick(now, r));
    assert!(run.s_events.contains(&S::EscalationEnded));
    assert!(!run.s_events.iter().any(|e| matches!(e, S::Stop(_))));
    run.host(|s, now, r| s.exit(0, now, r)).expect("exit");
    let rec = received(&run);
    let k = kinds(&rec);
    assert!(
        matches!(k[0], EntryBody::EscGranted { grant, expires_ms, .. } if *grant == esc_hash(&g) && expires_ms == ends)
    );
    assert_eq!(k[1], EntryBody::EscEnded { grant: &esc_hash(&g), why: 1 });
    assert!(matches!(check(&run, &rec, &[]).expect("checks").verdict, Verdict::Complete { .. }));
}
