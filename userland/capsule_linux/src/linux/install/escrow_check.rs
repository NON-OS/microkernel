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

//! The install role's own check of an escrow delivery's plaintext: the
//! bytes the meter hands over must be exactly content_len long and give the
//! pin's chunk_root, whatever the meter checked before. The tree is computed
//! by the caller, with B's nonos_nox_chunks at integration (I-C5), so there
//! is one copy of it. Pure, so the host proofs hold it.

use super::escrow_pin::EscrowPin;

/// Why the plaintext is not what the pin names.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NotPinned {
    /// Its length is not the pin's content_len.
    Length,
    /// Its chunk_root is not the pin's.
    Root,
}

/// Ok only when `plaintext` is what `pin` names; `chunk_root` computes the
/// root of the plan over it.
pub fn escrow_pinned(
    plaintext: &[u8],
    pin: &EscrowPin,
    chunk_root: impl FnOnce(&[u8]) -> [u8; 32],
) -> Result<(), NotPinned> {
    if u64::try_from(plaintext.len()).ok() != Some(pin.content_len) {
        return Err(NotPinned::Length);
    }
    match chunk_root(plaintext) == pin.chunk_root {
        true => Ok(()),
        false => Err(NotPinned::Root),
    }
}
