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

//! One role in a table: its name, its flags, how many approvals an
//! escalation needs, its longest session, its ceiling and its escalation
//! ceiling. A ceiling names only capabilities the kernel has, and never one
//! of the forbidden nine. Roles come in strictly ascending name order.

use crate::policy::{within, MAX_ROLES, MAX_SPAN_MS};
use crate::{CapSet, PolicyError};
use alloc::vec::Vec;
use nsh_wire::names::{self, MAX_ROLE};
use nsh_wire::{CapNames, Reader, WireError, MAX_APPROVALS};

/// The one flag defined: `record-commands: digest`.
const FLAG_DIGEST: u8 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Role<'a> {
    pub name: &'a [u8],
    /// The record keeps a digest of each command rather than its text.
    pub record_digest: bool,
    /// How many approvals an escalation needs: 0, 1 or 2.
    pub approvals: u8,
    pub max_session_ms: u64,
    pub ceiling: CapSet,
    pub escalation: CapSet,
}

pub(crate) fn roles<'a>(r: &mut Reader<'a>) -> Result<Vec<Role<'a>>, PolicyError> {
    let n = within(u64::from(r.u8()?), 1, MAX_ROLES as u64)? as usize;
    let mut out: Vec<Role<'a>> = Vec::with_capacity(n);
    for _ in 0..n {
        let role = role(r)?;
        if out.last().is_some_and(|before| role.name <= before.name) {
            return Err(PolicyError::Order);
        }
        out.push(role);
    }
    Ok(out)
}

fn role<'a>(r: &mut Reader<'a>) -> Result<Role<'a>, PolicyError> {
    let name = r.bytes8(1, MAX_ROLE)?;
    names::role(name)?;
    let flags = r.u8()?;
    if flags & !FLAG_DIGEST != 0 {
        return Err(WireError::Reserved.into());
    }
    let approvals = within(u64::from(r.u8()?), 0, MAX_APPROVALS as u64)? as u8;
    let max_session_ms = within(r.u64()?, 1, MAX_SPAN_MS)?;
    let ceiling = CapSet::ceiling(&CapNames::read(r)?)?;
    if ceiling.is_empty() {
        return Err(PolicyError::EmptyCeiling);
    }
    let escalation = CapSet::ceiling(&CapNames::read(r)?)?;
    let record_digest = flags & FLAG_DIGEST != 0;
    Ok(Role { name, record_digest, approvals, max_session_ms, ceiling, escalation })
}
