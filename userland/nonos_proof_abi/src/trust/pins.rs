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

//! What a verifier trusts, per release: the trees a device proof must walk,
//! the registrars' device roots and how long each is good for, the vendor
//! supervisors that may seal a run (their caps are pinned in caps.rs), the
//! vendor policies, and the isolation statement. Nothing here comes from a container.

use nox_verify::Statement;

/// One registrar's device registry: its root at the depth it was published,
/// good from `from_day` for `life_days` days.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct DeviceRoot {
    pub root: [u64; 4],
    pub depth: usize,
    pub from_day: u64,
    pub life_days: u64,
}

impl DeviceRoot {
    pub fn good_on(&self, day: u64) -> bool {
        day >= self.from_day && day - self.from_day <= self.life_days
    }
}

#[derive(Clone, Copy)]
pub struct Trust<'a> {
    pub boot_root: [u64; 4],
    /// Every kernel under it carries the seal syscalls.
    pub kernel_root: [u64; 4],
    pub registrars: &'a [DeviceRoot],
    /// The vendor capsule_linux measurements a sealing supervisor may have.
    pub supervisors: &'a [[u8; 32]],
    /// The vendor table's policies, for programs shipped with the system.
    pub vendor_policies: &'a [[u8; 256]],
    pub isolation: &'a Statement,
}
