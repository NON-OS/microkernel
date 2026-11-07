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
use super::good::{kinds, received};
use crate::kit::roles::Table;
use crate::kit::roles_build::grant::GrantB;
use crate::kit::roles_build::revoke::RevB;
use crate::kit::roles_build::ESC_ID;
use crate::kit::run::Run;
use nsh_server::Event as S;
use nsh_wire::EntryBody;

#[test]
fn a_revoked_escalation_ends_and_the_session_goes_on() {
    let table = Table::shipped();
    let handle = table.clone();
    let mut run = Run::new(&GrantB::default(), table);
    run.pump();
    run.user(|c| c.escalate(&GrantB::escalation().bytes())).expect("escalate");
    handle.install(&RevB { ids: vec![ESC_ID], ..RevB::default() });
    run.host(|s, now, r| s.tick(now, r));
    assert!(run.s_events.contains(&S::EscalationEnded));
    assert!(!run.s_events.iter().any(|e| matches!(e, S::Stop(_))));
    run.host(|s, now, r| s.exit(0, now, r)).expect("exit");
    let g = GrantB::escalation();
    let rec = received(&run);
    assert_eq!(kinds(&rec)[1], EntryBody::EscEnded { grant: &esc_hash(&g), why: 3 });
}

#[test]
fn the_session_ending_ends_its_escalation_first() {
    let mut run = Run::logged_in();
    run.user(|c| c.escalate(&GrantB::escalation().bytes())).expect("escalate");
    run.host(|s, now, r| s.exit(0, now, r)).expect("exit");
    let g = GrantB::escalation();
    let rec = received(&run);
    let k = kinds(&rec).split_off(1);
    assert_eq!(k, [EntryBody::EscEnded { grant: &esc_hash(&g), why: 2 }, EntryBody::Exit(0)]);
}
