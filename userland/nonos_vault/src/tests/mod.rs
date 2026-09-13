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

//! What sealing has to guarantee.
//!
//! The machine root comes from the TPM, so these drive the derivation with a
//! known root instead. That is the whole point of splitting the two: the
//! properties worth proving are about what the derivation does with a root,
//! not about where the root came from.

mod keys;
mod records;
mod refusals;

use crate::blob::OVERHEAD;
use crate::{open, seal, subkey};

pub(super) const ROOT: [u8; 32] = [0x5a; 32];
pub(super) const OTHER_ROOT: [u8; 32] = [0x5b; 32];
pub(super) const NONCE: [u8; 12] = [0x11; 12];

pub(super) fn sealed(record: &[u8], plaintext: &[u8]) -> alloc::vec::Vec<u8> {
    let mut out = alloc::vec![0u8; plaintext.len() + OVERHEAD];
    let n = seal(&ROOT, record, plaintext, &NONCE, &mut out).expect("seal");
    out.truncate(n);
    out
}
