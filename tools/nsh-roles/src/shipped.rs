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

//! The shipped roles, exactly as nsh_roles::shipped defines them.

use crate::roles_file::Role;
use nsh_roles::shipped::SHIPPED;

pub fn shipped() -> Vec<Role> {
    let names = |l: &[&str]| l.iter().map(|n| n.to_string()).collect();
    let role = |s: &nsh_roles::shipped::RoleSpec| Role {
        name: s.name.to_string(),
        digest: s.record_digest,
        approvals: s.approvals,
        max_session_ms: s.max_session_ms,
        ceiling: names(s.ceiling),
        escalation: names(s.escalation),
    };
    SHIPPED.iter().map(role).collect()
}
