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

//! Reading an enrolment certificate. Every field is placed and shaped here;
//! the signature, the expiry and whether the names match this connection
//! are the client's checks, made against its policy.

use crate::enrol::{fleet, Enrol, ENROL_MAGIC, ENROL_VERSION};
use crate::keys::{before_sig, HybridSig};
use crate::names::{self, MAX_HOST};
use crate::{Reader, WireError};

impl<'a> Enrol<'a> {
    pub fn parse(body: &'a [u8]) -> Result<Self, WireError> {
        let mut r = Reader::new(body);
        r.preamble(&ENROL_MAGIC, ENROL_VERSION)?;
        let fleet_id = fleet(r.array()?)?;
        let host = r.bytes8(1, MAX_HOST)?;
        names::host(host)?;
        let ak_public = r.array()?;
        let host_keys = r.array()?;
        let pcrs = [r.array()?, r.array()?, r.array()?, r.array()?];
        let boot_apps = crate::enrol::boot::read(&mut r)?;
        let not_after_ms = r.u64()?;
        let authority = r.array()?;
        let sig = HybridSig::read(&mut r)?;
        r.finish()?;
        let signed = before_sig(body)?;
        Ok(Enrol {
            fleet_id,
            host,
            ak_public,
            host_keys,
            pcrs,
            boot_apps,
            not_after_ms,
            authority,
            sig,
            signed,
        })
    }
}
