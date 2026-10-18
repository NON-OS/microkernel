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

extern crate alloc;

use alloc::vec::Vec;

use super::error::DecodeError;
use super::reader::Reader;
use super::strings::{bounded_bytes, bounded_count};
use crate::limits::{GUEST_POLICY_LEN, MAX_PROOF_ANCHORS};
use crate::types::ReleaseExt;

pub(super) fn read(r: &mut Reader<'_>) -> Result<ReleaseExt, DecodeError> {
    let commerce_hash = r.fixed::<32>()?;
    let guest_policy = bounded_bytes(r, GUEST_POLICY_LEN)?;
    if !guest_policy.is_empty() && guest_policy.len() != GUEST_POLICY_LEN as usize {
        return Err(DecodeError::BadLength);
    }
    let chunk_root = r.fixed::<32>()?;
    let content_len = r.u64()?;

    let n = bounded_count(r, MAX_PROOF_ANCHORS)?;
    let mut proof_anchors = Vec::with_capacity(n as usize);
    for _ in 0..n {
        proof_anchors.push(r.fixed::<32>()?);
    }
    let release_attest_sig = r.fixed::<65>()?;
    Ok(ReleaseExt {
        commerce_hash,
        guest_policy,
        chunk_root,
        content_len,
        proof_anchors,
        release_attest_sig,
    })
}
