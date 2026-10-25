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

//! Reading a role table. Everything the bytes alone settle is settled here:
//! sizes, ranges, order, name shapes and the ceilings' names. The signature
//! is `verify`'s.

use crate::policy::keys::{disjoint, keys};
use crate::policy::role::roles;
use crate::policy::{within, Table, MAX_APPROVERS, MAX_AUTHORITIES, MAX_REVOCATION_AGE_MS};
use crate::policy::{POLICY_MAGIC, POLICY_VERSION};
use crate::PolicyError;
use nsh_wire::enrol::fleet;
use nsh_wire::keys::{HybridSig, KEY_ID_LEN};
use nsh_wire::Reader;

/// A table as read, its signature not yet checked.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unverified<'a> {
    pub table: Table<'a>,
    pub signer: &'a [u8; KEY_ID_LEN],
    pub sig: HybridSig<'a>,
    /// The bytes the signatures cover: everything before them.
    pub signed: &'a [u8],
}

impl<'a> Unverified<'a> {
    pub fn parse(body: &'a [u8]) -> Result<Self, PolicyError> {
        let mut r = Reader::new(body);
        r.preamble(&POLICY_MAGIC, POLICY_VERSION)?;
        let fleet_id = fleet(r.array()?)?;
        let (serial, issued_ms) = (r.u64()?, r.u64()?);
        let revocation_max_age_ms = within(r.u64()?, 1, MAX_REVOCATION_AGE_MS)?;
        let authorities = keys(&mut r, 1, MAX_AUTHORITIES)?;
        let approvers = keys(&mut r, 0, MAX_APPROVERS)?;
        disjoint(&authorities, &approvers)?;
        let roles = roles(&mut r)?;
        let signer = r.array()?;
        let signed = &body[..body.len() - r.left()];
        let sig = HybridSig::read(&mut r)?;
        r.finish()?;
        let (max_age, auth, appr) = (revocation_max_age_ms, authorities, approvers);
        let table = Table {
            fleet_id,
            serial,
            issued_ms,
            revocation_max_age_ms: max_age,
            authorities: auth,
            approvers: appr,
            roles,
        };
        Ok(Unverified { table, signer, sig, signed })
    }
}
