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

//! A hybrid public key and a hybrid signature: Ed25519 and ML-DSA-65 side
//! by side, both always present. The parser only places the bytes; whether
//! a signature verifies is the caller's check, and both halves must pass.

use crate::{Reader, WireError, Writer};

pub const ED_PUBLIC_LEN: usize = 32;
pub const ED_SIG_LEN: usize = 64;
pub const ML_PUBLIC_LEN: usize = 1952;
pub const ML_SIG_LEN: usize = 3309;
pub const HYBRID_PUBLIC_LEN: usize = ED_PUBLIC_LEN + ML_PUBLIC_LEN;
pub const HYBRID_SIG_LEN: usize = ED_SIG_LEN + ML_SIG_LEN;
/// A key id or a digest that names a key: SHA-256 of the hybrid public key.
pub const KEY_ID_LEN: usize = 32;
/// A SHA-512 digest, as the transcript, the subject and the record use.
pub const DIGEST_LEN: usize = 64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HybridPublic<'a> {
    pub ed: &'a [u8; ED_PUBLIC_LEN],
    pub ml: &'a [u8; ML_PUBLIC_LEN],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HybridSig<'a> {
    pub ed: &'a [u8; ED_SIG_LEN],
    pub ml: &'a [u8; ML_SIG_LEN],
}

impl<'a> HybridPublic<'a> {
    pub fn read(r: &mut Reader<'a>) -> Result<Self, WireError> {
        Ok(HybridPublic { ed: r.array()?, ml: r.array()? })
    }

    pub fn write(&self, w: &mut Writer<'_>) -> Result<(), WireError> {
        w.put(self.ed)?;
        w.put(self.ml)
    }
}

impl<'a> HybridSig<'a> {
    pub fn read(r: &mut Reader<'a>) -> Result<Self, WireError> {
        Ok(HybridSig { ed: r.array()?, ml: r.array()? })
    }

    pub fn write(&self, w: &mut Writer<'_>) -> Result<(), WireError> {
        w.put(self.ed)?;
        w.put(self.ml)
    }
}

/// The bytes a body's trailing signature covers: everything before it.
pub fn before_sig(body: &[u8]) -> Result<&[u8], WireError> {
    let n = body.len().checked_sub(HYBRID_SIG_LEN).ok_or(WireError::Short)?;
    Ok(&body[..n])
}
