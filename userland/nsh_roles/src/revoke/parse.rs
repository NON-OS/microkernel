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

//! Reading a revocation list: the fleet, the issue time, the revoked grant
//! ids in strictly ascending order with none of them zero, the signer and
//! its signature. The signature is `verify`'s.

use crate::revoke::{MAX_REVOKED, REVOKE_MAGIC, REVOKE_VERSION};
use crate::PolicyError;
use nsh_wire::enrol::fleet;
use nsh_wire::keys::{HybridSig, KEY_ID_LEN};
use nsh_wire::{Reader, GRANT_ID_LEN};

/// A list as read, its signature not yet checked.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RevocationList<'a> {
    pub fleet_id: &'a [u8; 16],
    pub issued_ms: u64,
    pub(crate) ids: &'a [u8],
    pub signer: &'a [u8; KEY_ID_LEN],
    pub sig: HybridSig<'a>,
    /// The bytes the signatures cover: everything before them.
    pub signed: &'a [u8],
}

impl<'a> RevocationList<'a> {
    pub fn parse(body: &'a [u8]) -> Result<Self, PolicyError> {
        let mut r = Reader::new(body);
        r.preamble(&REVOKE_MAGIC, REVOKE_VERSION)?;
        let fleet_id = fleet(r.array()?)?;
        let issued_ms = r.u64()?;
        let n = usize::from(r.u16()?);
        if n > MAX_REVOKED {
            return Err(PolicyError::Range);
        }
        let ids = r.take(n * GRANT_ID_LEN)?;
        let mut before: Option<&[u8]> = None;
        for id in ids.chunks_exact(GRANT_ID_LEN) {
            if id.iter().all(|b| *b == 0) {
                return Err(PolicyError::Range);
            }
            if before.is_some_and(|b| id <= b) {
                return Err(PolicyError::Order);
            }
            before = Some(id);
        }
        let signer = r.array()?;
        let signed = &body[..body.len() - r.left()];
        let sig = HybridSig::read(&mut r)?;
        r.finish()?;
        Ok(RevocationList { fleet_id, issued_ms, ids, signer, sig, signed })
    }
}
