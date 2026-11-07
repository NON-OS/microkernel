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

//! A call the kernel refused the session, and whether the session runs.

use super::stage::Stage;
use super::Server;
use crate::record::EntryKind;
use crate::record::Unfit;
use crate::{Attestation, CallError, HostSigner, Rand, Roles};
use alloc::vec::Vec;

impl<A: Attestation, S: HostSigner, R: Roles> Server<A, S, R> {
    /// A call the kernel refused the session, by its syscall tag and the
    /// capability it lacked.
    pub fn refused_call(
        &mut self,
        tag: u32,
        cap: &[u8],
        now_ms: u64,
        rand: Rand<'_>,
    ) -> Result<(), CallError> {
        self.running()?;
        let n = u8::try_from(cap.len()).map_err(|_| CallError::Malformed)?;
        let mut body = Vec::with_capacity(5 + cap.len());
        body.extend_from_slice(&tag.to_le_bytes());
        body.push(n);
        body.extend_from_slice(cap);
        match self.entry(EntryKind::RefusedCall, now_ms, &body, false, rand) {
            Err(Unfit::Body) => Err(CallError::Malformed),
            _ => Ok(()),
        }
    }

    /// The session is running and its shell has not exited; whether its
    /// role keeps command digests rather than lines.
    pub(super) fn running(&self) -> Result<bool, CallError> {
        match (&self.stage, self.live.as_ref()) {
            (Stage::Session, Some(l)) => Ok(l.login.record_digest),
            _ => Err(CallError::NotNow),
        }
    }
}
