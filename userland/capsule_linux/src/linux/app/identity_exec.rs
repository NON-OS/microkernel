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

//! The box a guest runs in after an exec: the new program brings its own
//! policy, but runs with only what both it and the program that exec'd it
//! were given. So a shell with no network cannot exec its way onto one, and
//! a program held to a destination list cannot exec its way past it.

use nonos_linux_policy::mask::{narrow, NET_ANON};

use super::identity::AppIdentity;
use super::reach::{chain, ANYWHERE};

impl AppIdentity {
    /// `program`, exec'd by the guest this is: its identity, inside what
    /// both were allowed.
    pub fn exec(&self, program: &AppIdentity) -> Self {
        let mask = narrow(self.mask, program.mask);
        let (mask, reach) = match chain(&self.reach, &program.reach) {
            Some(reach) => (mask, reach),
            None => (mask & !NET_ANON, ANYWHERE),
        };
        Self { mask, calls: self.calls.meet(&program.calls), reach, ..*program }
    }
}
