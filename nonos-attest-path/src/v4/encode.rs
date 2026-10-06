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

use alloc::vec::Vec;

use super::layout::{MAGIC_V4, MAX_PROOF_V4};
use crate::leaf::Kind;
use crate::trailer::MAGIC;

/// Write a v4 trailer. `None` for a pad kind, a path that is not a v3
/// trailer, an empty or oversized proof, or a length that does not fit.
pub fn encode_v4(kind: Kind, path: &[u8], proof: &[u8]) -> Option<Vec<u8>> {
    if kind == Kind::Pad || !path.starts_with(&MAGIC) {
        return None;
    }
    if proof.is_empty() || proof.len() > MAX_PROOF_V4 {
        return None;
    }
    let path_len = u32::try_from(path.len()).ok()?;
    let proof_len = u32::try_from(proof.len()).ok()?;
    let mut out = Vec::with_capacity(13 + path.len() + 4 + proof.len());
    out.extend_from_slice(&MAGIC_V4);
    out.push(kind as u8);
    out.extend_from_slice(&path_len.to_le_bytes());
    out.extend_from_slice(path);
    out.extend_from_slice(&proof_len.to_le_bytes());
    out.extend_from_slice(proof);
    Some(out)
}
