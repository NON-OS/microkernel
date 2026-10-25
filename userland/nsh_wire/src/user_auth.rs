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

//! `USER_AUTH`: the user's hybrid public key, a base grant (never an
//! escalation), and their
//! signature over the transcript, which already holds `HOST_PROOF`. It is
//! sent only after the client has accepted the host.

use crate::cap_names::{MAX_CAPS, MAX_CAP_NAME};
use crate::grant::approval::{APPROVAL_LEN, MAX_APPROVALS};
use crate::keys::{before_sig, HybridPublic, HybridSig, HYBRID_SIG_LEN};
use crate::names::{MAX_ROLE, MAX_USER};
use crate::{Grant, Reader, WireError};

/// The largest `nsh-grant/1`, every list full and every name at its cap.
pub const MAX_GRANT: usize = 4
    + 2
    + 2
    + 3 * 16
    + 64
    + (1 + MAX_USER)
    + (1 + MAX_ROLE)
    + (1 + MAX_CAPS * (1 + MAX_CAP_NAME))
    + 8
    + 8
    + 32
    + HYBRID_SIG_LEN
    + 1
    + MAX_APPROVALS * APPROVAL_LEN;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UserAuth<'a> {
    pub user: HybridPublic<'a>,
    pub grant: Grant<'a>,
    pub sig: HybridSig<'a>,
    /// The bytes the transcript for the user signature covers.
    pub signed: &'a [u8],
}

impl<'a> UserAuth<'a> {
    pub fn parse(body: &'a [u8]) -> Result<Self, WireError> {
        let mut r = Reader::new(body);
        let user = HybridPublic::read(&mut r)?;
        let grant = Grant::parse(r.bytes16(1, MAX_GRANT)?)?;
        if grant.kind != crate::GrantKind::Base {
            return Err(WireError::Value);
        }
        let sig = HybridSig::read(&mut r)?;
        r.finish()?;
        Ok(UserAuth { user, grant, sig, signed: before_sig(body)? })
    }
}
