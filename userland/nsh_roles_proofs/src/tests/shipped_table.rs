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

//! A table made of the shipped roles opens, and holds them as written.

use crate::build::table::TableB;
use nsh_roles::open_policy;
use nsh_roles::shipped::SHIPPED;

#[test]
fn a_table_of_the_shipped_roles_opens_as_written() {
    let t = TableB::default();
    let bytes = t.bytes();
    let policy = open_policy(&bytes, &t.anchor, 0).expect("opens");
    assert_eq!(policy.roles.len(), 3);
    for (role, spec) in policy.roles.iter().zip(&SHIPPED) {
        assert_eq!(role.name, spec.name.as_bytes());
        assert_eq!(role.ceiling.names(), spec.ceiling);
        assert_eq!(role.escalation.names(), spec.escalation);
        assert_eq!(role.approvals, spec.approvals);
        assert_eq!(role.max_session_ms, spec.max_session_ms);
        assert_eq!(role.record_digest, spec.record_digest);
    }
}
