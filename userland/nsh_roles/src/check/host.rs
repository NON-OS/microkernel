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

//! What the server knows when it checks a grant: its trusted role table,
//! its trusted revocation list, and the time now, which the caller reads
//! from the host clock. Nothing in this crate reads a clock.

use crate::{Policy, PolicyError, Refusal, Revocations, Role};
use nsh_wire::GRANT_ID_LEN;

pub struct Host<'h> {
    pub(crate) policy: &'h Policy<'h>,
    pub(crate) revocations: &'h Revocations<'h>,
    pub(crate) now_ms: u64,
}

impl<'h> Host<'h> {
    /// The list must be one this table trusts: same fleet, and signed by
    /// one of its authorities.
    pub fn new(
        p: &'h Policy<'h>,
        r: &'h Revocations<'h>,
        now_ms: u64,
    ) -> Result<Self, PolicyError> {
        let list = r.list();
        if list.fleet_id != p.fleet_id {
            return Err(PolicyError::Fleet);
        }
        p.authority(list.signer).ok_or(PolicyError::Signer)?;
        Ok(Host { policy: p, revocations: r, now_ms })
    }
}

/// The list is fresh, and names none of `ids`.
pub(crate) fn revocation(host: &Host<'_>, ids: &[&[u8; GRANT_ID_LEN]]) -> Result<(), Refusal> {
    if !host.revocations.fresh(host.now_ms, host.policy.revocation_max_age_ms) {
        return Err(Refusal::RevocationStale);
    }
    match ids.iter().any(|id| host.revocations.contains(id)) {
        true => Err(Refusal::Revoked),
        false => Ok(()),
    }
}

pub(crate) fn role<'p>(host: &'p Host<'_>, name: &[u8]) -> Result<&'p Role<'p>, Refusal> {
    host.policy.role(name).ok_or(Refusal::RoleUnknown)
}
