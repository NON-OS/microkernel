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

//! Running the checks: a setting holds the table, the list and the clock;
//! each call opens the table and the list as a host does, then checks.

use super::grant::GrantB;
use super::revoke::RevB;
use super::table::TableB;
use super::user_auth::UserAuthB;
use super::{NOW, TH};
use nsh_roles::{login, open_policy, open_revocations, Host, Login, Refusal};
use nsh_wire::UserAuth;

#[derive(Debug, Clone)]
pub struct Setup {
    pub table: TableB,
    pub rev: RevB,
    pub now: u64,
}

impl Default for Setup {
    fn default() -> Self {
        Setup { table: TableB::default(), rev: RevB::default(), now: NOW }
    }
}

impl Setup {
    pub(super) fn with_host<T>(&self, f: impl FnOnce(&Host<'_>) -> T) -> T {
        let tb = self.table.bytes();
        let policy = open_policy(&tb, &self.table.anchor, 0).expect("the table opens");
        let rb = self.rev.bytes();
        let revs = open_revocations(&rb, &policy, 0).expect("the list opens");
        f(&Host::new(&policy, &revs, self.now).expect("the host"))
    }

    pub fn login_auth(&self, ua: &UserAuth<'_>) -> Result<Login, Refusal> {
        self.with_host(|host| login(host, ua, &TH))
    }

    pub fn login_ua(&self, ua: &UserAuthB) -> Result<Login, Refusal> {
        let bytes = ua.bytes();
        self.login_auth(&UserAuth::parse(&bytes).expect("USER_AUTH parses"))
    }

    pub fn login(&self, g: &GrantB) -> Result<Login, Refusal> {
        self.login_ua(&UserAuthB::new(g.bytes()))
    }

    /// The default base login, under the default table and list at `NOW`.
    pub fn base() -> Login {
        Setup::default().login(&GrantB::default()).expect("the base login")
    }
}
