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

//! An escalation needs a fresh list naming neither it nor its base grant,
//! the base role still in the table, and capabilities within that role's
//! escalation ceiling.

use crate::build::grant::GrantB;
use crate::build::revoke::RevB;
use crate::build::run::Setup;
use crate::build::{BASE_ID, ESC_ID, HOUR, NOW};
use nsh_roles::Refusal;

fn with_list(rev: RevB) -> Result<(), Refusal> {
    let s = Setup { rev, ..Setup::default() };
    s.escalate(&Setup::base(), &GrantB::escalation()).map(|_| ())
}

#[test]
fn a_stale_list_refuses_an_escalation() {
    let r = RevB { issued_ms: NOW - 24 * HOUR - 1, ..RevB::default() };
    assert_eq!(with_list(r), Err(Refusal::RevocationStale));
}

#[test]
fn an_escalation_on_the_list_or_under_a_revoked_base_is_refused() {
    let r = |id| RevB { ids: vec![id], ..RevB::default() };
    assert_eq!(with_list(r(ESC_ID)), Err(Refusal::Revoked));
    assert_eq!(with_list(r(BASE_ID)), Err(Refusal::Revoked));
}

#[test]
fn the_base_role_must_still_be_in_the_table() {
    let mut s = Setup::default();
    s.table.roles.retain(|r| r.name != b"operator");
    let e = s.escalate(&Setup::base(), &GrantB::escalation());
    assert_eq!(e.err(), Some(Refusal::RoleUnknown));
}
