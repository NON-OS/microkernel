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

//! A valid `nsh-policy/1` role table: the shipped roles, two authorities,
//! three approvers, signed by the authority the host pins.

use super::keys::{key, K};
pub use super::role::RoleB;
use super::{Spoil, FLEET, HOUR, NOW};

/// The default table's serial.
pub const SERIAL: u64 = 5;
use nsh_roles::shipped::SHIPPED;

#[derive(Debug, Clone)]
pub struct TableB {
    pub fleet: [u8; 16],
    pub serial: u64,
    pub issued_ms: u64,
    pub max_age_ms: u64,
    pub authorities: Vec<K>,
    pub approvers: Vec<K>,
    /// Lay key lists out in ascending key-id order, as the format requires.
    pub sort_keys: bool,
    pub roles: Vec<RoleB>,
    pub signer_id: [u8; 32],
    pub signer: K,
    pub spoil: Spoil,
    /// The key id the host pins.
    pub anchor: [u8; 32],
}

impl Default for TableB {
    fn default() -> Self {
        TableB {
            fleet: FLEET,
            serial: SERIAL,
            issued_ms: NOW - 2 * HOUR,
            max_age_ms: 24 * HOUR,
            authorities: vec![K::AuthA, K::AuthB],
            approvers: vec![K::Appr1, K::Appr2, K::Appr3],
            sort_keys: true,
            roles: SHIPPED.iter().map(RoleB::from).collect(),
            signer_id: key(K::AuthA).id(),
            signer: K::AuthA,
            spoil: Spoil::None,
            anchor: key(K::AuthA).id(),
        }
    }
}

impl TableB {
    pub fn role(&mut self, name: &str) -> &mut RoleB {
        self.roles.iter_mut().find(|r| r.name == name.as_bytes()).expect("role")
    }
}
