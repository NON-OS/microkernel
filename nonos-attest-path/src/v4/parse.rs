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

use super::layout::{HEAD, KIND_AT, MAGIC_V4, MAX_PROOF_V4, PATH_LEN_AT};
use crate::leaf::Kind;
use crate::trailer::MAGIC;

/// A v4 trailer as read: borrowed slices into the caller's bytes.
#[derive(Clone, Copy, Debug)]
pub struct TrailerV4<'a> {
    pub kind: Kind,
    pub path: &'a [u8],
    pub proof: &'a [u8],
}

/// Read `t` as a v4 trailer of kind `expect`, with a proof of at most
/// `max_proof` bytes. `None` for any other kind, any length out of bound,
/// any inner path that is not a v3 trailer, and any byte past the proof.
pub fn parse_v4(t: &[u8], expect: Kind, max_proof: usize) -> Option<TrailerV4<'_>> {
    if t.get(..8)? != MAGIC_V4 || max_proof == 0 || max_proof > MAX_PROOF_V4 {
        return None;
    }
    let kind = kind_of(*t.get(KIND_AT)?)?;
    if kind != expect {
        return None;
    }
    let path_len = word(t, PATH_LEN_AT)?;
    let path_end = HEAD.checked_add(path_len)?;
    let path = t.get(HEAD..path_end)?;
    if !path.starts_with(&MAGIC) {
        return None;
    }
    let proof_len = word(t, path_end)?;
    if proof_len == 0 || proof_len > max_proof {
        return None;
    }
    let proof_at = path_end.checked_add(4)?;
    if t.len() != proof_at.checked_add(proof_len)? {
        return None;
    }
    Some(TrailerV4 { kind, path, proof: t.get(proof_at..)? })
}

fn kind_of(b: u8) -> Option<Kind> {
    match b {
        0 => Some(Kind::Kernel),
        1 => Some(Kind::Capsule),
        3 => Some(Kind::Bootloader),
        _ => None,
    }
}

fn word(t: &[u8], at: usize) -> Option<usize> {
    let b = t.get(at..at.checked_add(4)?)?;
    usize::try_from(u32::from_le_bytes([b[0], b[1], b[2], b[3]])).ok()
}
