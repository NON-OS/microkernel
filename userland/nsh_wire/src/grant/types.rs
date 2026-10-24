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

//! A role grant, `nsh-grant/1` (docs/design/nsh-v1-roles.md). A base grant
//! names a user's role for a while; an escalation grant names more, for at
//! most an hour, under a base grant, and may carry approvals from others.

use crate::keys::{HybridSig, DIGEST_LEN, KEY_ID_LEN};
use crate::CapNames;

pub const GRANT_MAGIC: [u8; 4] = *b"NSHG";
pub const GRANT_VERSION: u16 = 1;
pub const GRANT_ID_LEN: usize = 16;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GrantKind {
    Base = 1,
    Escalation = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Grant<'a> {
    pub kind: GrantKind,
    pub grant_id: &'a [u8; GRANT_ID_LEN],
    /// All zeros for a base grant; the base grant's id for an escalation.
    pub parent_id: &'a [u8; GRANT_ID_LEN],
    pub fleet_id: &'a [u8; 16],
    /// SHA-512 of the user's hybrid public key.
    pub subject: &'a [u8; DIGEST_LEN],
    pub user: &'a [u8],
    pub role: &'a [u8],
    pub caps: CapNames<'a>,
    pub not_before_ms: u64,
    pub not_after_ms: u64,
    pub issuer: &'a [u8; KEY_ID_LEN],
    pub sig: HybridSig<'a>,
    /// The bytes the issuer signed.
    pub signed: &'a [u8],
    /// The signed bytes and the issuer's signature: what the grant's hash
    /// covers. Approvals are left out, since the issuer did not sign them.
    pub issued: &'a [u8],
    /// The approvals, read with `approvals()`.
    pub(crate) approvals: &'a [u8],
}
