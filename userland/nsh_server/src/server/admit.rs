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

//! The host's answer to an admitted login. `begin` names the capabilities
//! the session capsule was actually given, which may be fewer than the
//! grant's but never more; `AUTH_OK` states them, the record's header is
//! written, and the session keys replace the handshake keys. `deny` ends
//! the connection with `ABORT(BUSY)`.

use super::stage::Stage;
use super::Server;
use crate::{Attestation, CallError, HostSigner, Rand, Roles, Why};
use alloc::vec::Vec;
use nsh_roles::CapSet;
use nsh_wire::{AuthOk, WireError, SESSION_ID_LEN};

impl<A: Attestation, S: HostSigner, R: Roles> Server<A, S, R> {
    pub fn begin(&mut self, caps: CapSet, now_ms: u64, rand: Rand<'_>) -> Result<(), CallError> {
        if self.stage != Stage::Admitting {
            return Err(CallError::NotNow);
        }
        if !self.admitted.as_ref().is_some_and(|l| !caps.is_empty() && caps.within(l.caps)) {
            return Err(CallError::Beyond);
        }
        if let Err(why) = self.start(caps, now_ms, rand) {
            self.refuse(why, now_ms, rand);
        }
        Ok(())
    }

    pub fn deny(&mut self, now_ms: u64, rand: Rand<'_>) -> Result<(), CallError> {
        if self.stage != Stage::Admitting {
            return Err(CallError::NotNow);
        }
        self.refuse(Why::Denied, now_ms, rand);
        Ok(())
    }
}

/// `AUTH_OK`'s body: the session id, the idle timeout and the capabilities.
pub(super) fn auth_ok(
    id: &[u8; SESSION_ID_LEN],
    idle_s: u32,
    caps: &[&[u8]],
) -> Result<Vec<u8>, WireError> {
    let mut body = alloc::vec![0u8; SESSION_ID_LEN + 4 + 1 + caps.len() * 33];
    let n = AuthOk::encode(id, idle_s, caps, &mut body)?;
    body.truncate(n);
    Ok(body)
}
