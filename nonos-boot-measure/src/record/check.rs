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

use sha2::{Digest, Sha256};

use super::error::RecordError;
use super::parse::{parse, Record};
use super::DOMAIN;

/// What the release key signs for a root at an epoch.
pub fn message(root: &[u8; 32], epoch: u64) -> [u8; 32] {
    Sha256::new()
        .chain_update(DOMAIN)
        .chain_update(root)
        .chain_update(epoch.to_le_bytes())
        .finalize()
        .into()
}

/*
 * The whole check, in the order that never asks the verifier about a record
 * that is already refused: the layout, the signature over the message, then
 * the epoch against the floor the TPM holds. `verify(digest, r, s)` is the
 * caller's ECDSA P-256 check under the release key it trusts.
 */
pub fn check(
    b: &[u8],
    floor: u64,
    verify: impl FnOnce(&[u8; 32], &[u8; 32], &[u8; 32]) -> bool,
) -> Result<Record, RecordError> {
    let rec = parse(b)?;
    if !verify(&message(&rec.root, rec.epoch), &rec.r, &rec.s) {
        return Err(RecordError::BadSignature);
    }
    if rec.epoch < floor {
        return Err(RecordError::Stale);
    }
    Ok(rec)
}
