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

//! Running an escalation, under a setting's table, list and clock, from a
//! base login.

use super::grant::GrantB;
use super::run::Setup;
use nsh_roles::{escalate, Escalation, Login, Refusal};
use nsh_wire::Grant;

impl Setup {
    pub fn escalate_grant(
        &self,
        base: &Login,
        active: bool,
        g: &Grant<'_>,
    ) -> Result<Escalation, Refusal> {
        self.with_host(|host| escalate(host, base, active, g))
    }

    pub fn escalate(&self, base: &Login, g: &GrantB) -> Result<Escalation, Refusal> {
        let bytes = g.bytes();
        let grant = Grant::parse(&bytes).expect("the grant parses");
        self.escalate_grant(base, false, &grant)
    }
}
