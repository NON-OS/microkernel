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

//! The encoder refuses anything the decoder would, so a blob it writes
//! always reads back.

use super::decode_commerce::known_proof_kind;
use super::encode_error::EncodeError;
use crate::limits::MAX_RELEASES;
use crate::limits::{GUEST_POLICY_LEN, MAX_ENTRIES, MAX_PROOF_ANCHORS, MAX_PROOF_REQS};
use crate::types::{MarketplaceEntry, MarketplaceIndex, SettleMode};

pub(super) fn check(index: &MarketplaceIndex) -> Result<(), EncodeError> {
    if index.entries.len() > MAX_ENTRIES as usize {
        return Err(EncodeError::TooMany);
    }
    index.entries.iter().try_for_each(check_entry)
}

fn check_entry(e: &MarketplaceEntry) -> Result<(), EncodeError> {
    let c = e.commerce.as_ref().ok_or(EncodeError::MissingCommerce)?;
    if c.proof_reqs.len() > MAX_PROOF_REQS as usize || e.releases.len() > MAX_RELEASES as usize {
        return Err(EncodeError::TooMany);
    }
    if !c.proof_reqs.iter().all(|r| known_proof_kind(r.kind)) {
        return Err(EncodeError::UnknownKind);
    }
    for rel in &e.releases {
        let x = rel.ext.as_ref().ok_or(EncodeError::MissingReleaseExt)?;
        let policy = x.guest_policy.len();
        if policy != 0 && policy != GUEST_POLICY_LEN as usize {
            return Err(EncodeError::BadLength);
        }
        if x.proof_anchors.len() > MAX_PROOF_ANCHORS as usize {
            return Err(EncodeError::TooMany);
        }
        let pinned = x.chunk_root != [0; 32] && x.content_len != 0;
        if c.settle_mode == SettleMode::Escrow && !pinned {
            return Err(EncodeError::Inconsistent);
        }
    }
    Ok(())
}
