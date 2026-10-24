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

//! The approvals an escalation grant carries: each is an approver's key id
//! and that approver's hybrid signature over the issuer's signed bytes. A
//! base grant carries none; an approval comes from someone other than the
//! issuer, and two approvals name two approvers in ascending order, so one
//! set of approvals has one encoding.

use crate::grant::types::GrantKind;
use crate::keys::{HybridSig, HYBRID_SIG_LEN, KEY_ID_LEN};
use crate::{Grant, Reader, WireError};

pub const MAX_APPROVALS: usize = 2;
pub const APPROVAL_LEN: usize = KEY_ID_LEN + HYBRID_SIG_LEN;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Approval<'a> {
    pub approver: &'a [u8; KEY_ID_LEN],
    pub sig: HybridSig<'a>,
}

impl<'a> Grant<'a> {
    pub fn approvals(&self) -> impl Iterator<Item = Approval<'a>> + 'a {
        let mut r = Reader::new(self.approvals);
        core::iter::from_fn(move || match r.left() {
            0 => None,
            _ => Approval::read(&mut r).ok(),
        })
    }
}

impl<'a> Approval<'a> {
    pub fn read(r: &mut Reader<'a>) -> Result<Self, WireError> {
        Ok(Approval { approver: r.array()?, sig: HybridSig::read(r)? })
    }
}

pub(crate) fn approvals<'a>(
    r: &mut Reader<'a>,
    kind: GrantKind,
    issuer: &[u8; KEY_ID_LEN],
) -> Result<&'a [u8], WireError> {
    let count = usize::from(r.u8()?);
    if count > MAX_APPROVALS || (kind == GrantKind::Base && count > 0) {
        return Err(WireError::Value);
    }
    let start = r.pos;
    let mut before: Option<&[u8; KEY_ID_LEN]> = None;
    for _ in 0..count {
        let a = Approval::read(r)?;
        if a.approver == issuer || before.is_some_and(|b| a.approver <= b) {
            return Err(WireError::Value);
        }
        before = Some(a.approver);
    }
    Ok(&r.buf[start..r.pos])
}
