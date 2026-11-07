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

//! The host's role table and revocation list, built by nsh_roles_proofs'
//! builders and held by the server's own `RoleStore`, shared with the proof
//! so it can install a new list mid-session, or make the store fail, as a
//! host that can no longer vouch for its list would.

use super::roles_build::revoke::RevB;
use super::roles_build::table::TableB;
use nsh_roles::{Policy, PolicyError, Revocations};
use nsh_server::{RoleStore, Roles};
use std::cell::Cell;
use std::rc::Rc;

#[derive(Clone)]
pub struct Table {
    pub store: Rc<RoleStore>,
    /// While set, every check is refused with this.
    pub fail: Rc<Cell<Option<PolicyError>>>,
}

impl Table {
    pub fn new(table: &TableB, rev: &RevB) -> Self {
        let store = RoleStore::new(table.bytes(), table.anchor, 0, rev.bytes()).expect("the store");
        Table { store: Rc::new(store), fail: Rc::new(Cell::new(None)) }
    }

    pub fn shipped() -> Self {
        Table::new(&TableB::default(), &RevB::default())
    }

    /// Install `rev` as the host's list.
    pub fn install(&self, rev: &RevB) {
        self.store.install_list(rev.bytes()).expect("the list installs");
    }
}

impl Roles for Table {
    fn with<T>(
        &self,
        f: impl FnOnce(&Policy<'_>, &Revocations<'_>) -> T,
    ) -> Result<T, PolicyError> {
        match self.fail.get() {
            Some(e) => Err(e),
            None => self.store.with(f),
        }
    }
}
