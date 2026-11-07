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
use nsh_roles::PolicyError;
use nsh_server::Why;
use nsh_wire::Reason;

#[test]
fn a_host_that_cannot_vouch_for_its_list_refuses_every_login_as_stale() {
    let table = Table::shipped();
    table.fail.set(Some(PolicyError::Signature));
    let got = refused(GrantB::default(), table);
    assert_eq!(got, (Reason::RevocationStale, vec![Why::Policy(PolicyError::Signature)]));
}
