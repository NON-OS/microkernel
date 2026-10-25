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

//! Reading a grant. Everything the bytes alone can settle is settled here
//! (`grant::rules`); signatures, the clock, the fleet and the role table are
//! the server's checks.

use crate::enrol::fleet;
use crate::grant::approval::approvals;
use crate::grant::rules::{head, named, nonzero, parent};
use crate::grant::types::Grant;
use crate::keys::HybridSig;
use crate::names::{self, MAX_ROLE, MAX_USER};
use crate::{CapNames, Reader, WireError};

impl<'a> Grant<'a> {
    pub fn parse(body: &'a [u8]) -> Result<Self, WireError> {
        let mut r = Reader::new(body);
        let kind = head(&mut r)?;
        let grant_id = nonzero(r.array()?)?;
        let parent_id = parent(kind, grant_id, r.array()?)?;
        let fleet_id = fleet(r.array()?)?;
        let subject = r.array()?;
        let user = named(r.bytes8(1, MAX_USER)?, names::user)?;
        let role = named(r.bytes8(1, MAX_ROLE)?, names::role)?;
        let caps = CapNames::read(&mut r)?;
        let not_before_ms = r.u64()?;
        let not_after_ms = r.u64()?;
        if caps.is_empty() || not_before_ms >= not_after_ms {
            return Err(WireError::Value);
        }
        let issuer = r.array()?;
        let signed = &body[..r.pos];
        let sig = HybridSig::read(&mut r)?;
        let issued = &body[..r.pos];
        let approvals = approvals(&mut r, kind, issuer)?;
        r.finish()?;
        Ok(Grant {
            kind,
            grant_id,
            parent_id,
            fleet_id,
            subject,
            user,
            role,
            caps,
            not_before_ms,
            not_after_ms,
            issuer,
            sig,
            signed,
            issued,
            approvals,
        })
    }
}
