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

//! One role as a table lays it out, by default one of the shipped roles.

use nsh_roles::shipped::RoleSpec;

#[derive(Debug, Clone)]
pub struct RoleB {
    pub name: Vec<u8>,
    pub flags: u8,
    pub approvals: u8,
    pub max_session_ms: u64,
    pub ceiling: Vec<&'static str>,
    pub escalation: Vec<&'static str>,
}

impl From<&RoleSpec> for RoleB {
    fn from(s: &RoleSpec) -> Self {
        RoleB {
            name: s.name.as_bytes().to_vec(),
            flags: u8::from(s.record_digest),
            approvals: s.approvals,
            max_session_ms: s.max_session_ms,
            ceiling: s.ceiling.to_vec(),
            escalation: s.escalation.to_vec(),
        }
    }
}
