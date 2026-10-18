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
//! A shared device's interfaces and the pipes each driver configured.

use super::authorize::Authority;
use super::claims::Claims;
use super::holders::Holders;
use super::owners::Owners;

#[derive(Default)]
pub struct Sharing {
    pub claims: Claims,
    pub owners: Owners,
    pub holders: Holders,
    /// Which devices may be bound (authorize.rs).
    pub auth: Authority,
}

impl Sharing {
    /// Slot id `slot` names nothing now (its controller was revived): its
    /// claims, pipes, holder and standing go, so a device the id is given to
    /// next starts with none of them.
    pub fn forget(&mut self, slot: u8) {
        self.claims.list.retain(|c| c.slot != slot);
        self.owners.forget(slot);
        self.holders.forget(slot);
        self.auth.forget(slot);
    }
}
