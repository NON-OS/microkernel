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

//! The host's role table, `nsh-policy/1`. `Unverified::parse` reads the
//! bytes exactly; only `Unverified::verify`, under the key id the host
//! pins, makes a `Policy` the checks accept. The layout is in this crate's
//! README.

mod keys;
mod parse;
mod role;
mod verify;

use crate::hybrid::Key;
use alloc::vec::Vec;
use nsh_wire::keys::KEY_ID_LEN;

pub use parse::Unverified;
pub use role::Role;
pub use verify::{open_policy, Policy};

pub const POLICY_MAGIC: [u8; 4] = *b"NSHP";
pub const POLICY_VERSION: u16 = 1;
pub const MAX_AUTHORITIES: usize = 8;
pub const MAX_APPROVERS: usize = 16;
pub const MAX_ROLES: usize = 16;
/// The longest session a table may state.
pub const MAX_SPAN_MS: u64 = 7 * 24 * 3_600_000;
/// The longest revocation age a table may state; it may state less.
pub const MAX_REVOCATION_AGE_MS: u64 = 24 * 3_600_000;

/// What a table names, whether or not its signature has been checked.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Table<'a> {
    pub fleet_id: &'a [u8; 16],
    /// Strictly increasing from one table to the next it replaces.
    pub serial: u64,
    pub issued_ms: u64,
    pub revocation_max_age_ms: u64,
    pub authorities: Vec<Key<'a>>,
    pub approvers: Vec<Key<'a>>,
    pub roles: Vec<Role<'a>>,
}

impl<'a> Table<'a> {
    pub fn authority(&self, id: &[u8; KEY_ID_LEN]) -> Option<&Key<'a>> {
        self.authorities.iter().find(|k| &k.id == id)
    }

    pub fn approver(&self, id: &[u8; KEY_ID_LEN]) -> Option<&Key<'a>> {
        self.approvers.iter().find(|k| &k.id == id)
    }

    pub fn role(&self, name: &[u8]) -> Option<&Role<'a>> {
        self.roles.iter().find(|r| r.name == name)
    }
}

/// A count or a span within `min..=max`.
pub(crate) fn within(n: u64, min: u64, max: u64) -> Result<u64, crate::PolicyError> {
    (min..=max).contains(&n).then_some(n).ok_or(crate::PolicyError::Range)
}
