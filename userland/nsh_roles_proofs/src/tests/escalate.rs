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

//! A good escalation is admitted with its own capabilities and an end no
//! later than the base session's; only one runs at a time; it must be an
//! escalation grant, from a role authority whose signatures both verify.

use crate::build::grant::GrantB;
use crate::build::run::Setup;
use crate::build::{ESC_ID, HOUR, NOW};
use nsh_roles::Refusal;
use nsh_wire::{Grant, Reason};

pub fn esc(g: GrantB) -> Result<(), Refusal> {
    Setup::default().escalate(&Setup::base(), &g).map(|_| ())
}

#[test]
fn a_good_escalation_is_admitted() {
    let g = GrantB::escalation();
    let e = Setup::default().escalate(&Setup::base(), &g).expect("admitted");
    assert_eq!(e.grant_id, ESC_ID);
    assert_eq!(e.caps.names(), ["CoreExec", "IPC", "Memory", "Network"]);
    assert_eq!(e.ends_ms, NOW + HOUR / 2);
    let bytes = g.bytes();
    assert_eq!(e.hash, nsh_roles::grant_hash(&Grant::parse(&bytes).expect("parses")));
}

#[test]
fn an_escalation_never_outlives_its_base_session() {
    let s = Setup::default();
    let base = s.login(&GrantB { not_after_ms: NOW + HOUR / 4, ..GrantB::default() });
    let e = s.escalate(&base.expect("base"), &GrantB::escalation()).expect("admitted");
    assert_eq!(e.ends_ms, NOW + HOUR / 4);
}

#[test]
fn only_one_escalation_runs_at_a_time() {
    let bytes = GrantB::escalation().bytes();
    let g = Grant::parse(&bytes).expect("parses");
    let s = Setup::default();
    assert_eq!(s.escalate_grant(&Setup::base(), true, &g).err(), Some(Refusal::Active));
    assert!(s.escalate_grant(&Setup::base(), false, &g).is_ok());
    assert_eq!(Refusal::Active.reason(), Reason::EscActive);
}
