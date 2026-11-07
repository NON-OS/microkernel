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

//! `USER_AUTH` and the role table: each refusal is answered `AUTH_REFUSED`
//! with the one code it is told as, and logged whole; nothing about the
//! grant is judged before the proof of the key.

use super::login::*;
use crate::kit::roles::Table;
use crate::kit::roles_build::grant::GrantB;
use crate::kit::roles_build::revoke::RevB;
use crate::kit::roles_build::table::TableB;
use crate::kit::roles_build::{BASE_ID, HOUR, NOW};
use nsh_roles::Refusal as R;
use nsh_server::Why;
use nsh_wire::Reason;

#[test]
fn a_role_the_table_does_not_name_is_unknown() {
    grant_refused(
        GrantB { role: b"nobody".to_vec(), ..GrantB::default() },
        Reason::RoleUnknown,
        R::RoleUnknown,
    );
}

#[test]
fn capabilities_are_held_to_the_role() {
    let over = GrantB { caps: vec!["CoreExec", "IPC", "Network"], ..GrantB::default() };
    grant_refused(over, Reason::CapsCeiling, R::CapsCeiling);
    let unknown = GrantB { caps: vec!["CoreExec", "Nosuchcap"], ..GrantB::default() };
    grant_refused(unknown, Reason::CapUnknown, R::CapUnknown);
}

#[test]
fn a_revoked_grant_is_refused() {
    let table = Table::new(&TableB::default(), &RevB { ids: vec![BASE_ID], ..RevB::default() });
    assert_eq!(
        refused(GrantB::default(), table),
        (Reason::GrantRevoked, vec![Why::Login(R::Revoked)])
    );
}

#[test]
fn a_stale_list_refuses_every_login() {
    let rev = RevB { issued_ms: NOW - 25 * HOUR, ..RevB::default() };
    let table = Table::new(&TableB::default(), &rev);
    assert_eq!(
        refused(GrantB::default(), table),
        (Reason::RevocationStale, vec![Why::Login(R::RevocationStale)])
    );
}
