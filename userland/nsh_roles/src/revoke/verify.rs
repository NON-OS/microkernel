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

//! Trusting a revocation list: it is for the table's fleet, and signed by
//! one of the table's role authorities over
//! `H("nsh/1 revoke\0" || signed bytes)`, both halves verifying, and no
//! older than the newest list this host has accepted.

use crate::hybrid::verify;
use crate::revoke::{RevocationList, Revocations};
use crate::{Policy, PolicyError};
use nsh_crypto::{labelled, Label};

impl<'a> RevocationList<'a> {
    /// `min_issued_ms` is the issue time of the newest list this host has
    /// accepted; an older list is refused, the same one may be reloaded.
    pub fn verify(
        self,
        policy: &Policy<'_>,
        min_issued_ms: u64,
    ) -> Result<Revocations<'a>, PolicyError> {
        if self.fleet_id != policy.fleet_id {
            return Err(PolicyError::Fleet);
        }
        let key = policy.authority(self.signer).ok_or(PolicyError::Signer)?;
        if !verify(&key.public, &labelled(Label::Revoke, self.signed), &self.sig) {
            return Err(PolicyError::Signature);
        }
        if self.issued_ms < min_issued_ms {
            return Err(PolicyError::Replayed);
        }
        Ok(Revocations(self))
    }
}

/// Read a list and verify it against a trusted table, in one step.
pub fn open_revocations<'a>(
    body: &'a [u8],
    policy: &Policy<'_>,
    min_issued_ms: u64,
) -> Result<Revocations<'a>, PolicyError> {
    RevocationList::parse(body)?.verify(policy, min_issued_ms)
}
