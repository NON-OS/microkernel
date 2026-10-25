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

//! Machine keys bound to the capsule that asks.
//!
//! An unbound label names one key for every capsule holding Crypto. A label
//! that starts with BOUND_PREFIX names a key for its caller alone: the kernel
//! derives over the label followed by BLAKE3 of the caller's manifest
//! namespace, recorded at spawn. Two capsules naming the same bound label get
//! different keys, and none can name another's. The hash is a fixed length at
//! the end, so no two (label, namespace) pairs give the same bytes, and the
//! prefix keeps every bound label apart from the unbound and kernel ones.

extern crate alloc;

use alloc::vec::Vec;

use super::consts::LABEL_MAX;

/// The first byte of a bound label, beside the kernel's own prefix, 0.
pub const BOUND_PREFIX: u8 = 1;
/// The longest bound label a caller may name; the namespace hash follows it.
pub(crate) const BOUND_LABEL_MAX: usize = LABEL_MAX - 32;
const EPERM: i32 = 1;
const EINVAL: i32 = 22;

/// What the TPM derives for a caller's `label`. `namespace` is the caller's
/// namespace hash, which only a capsule the vendor root proved has. An
/// unbound label passes through unchanged, byte for byte.
pub fn bound_label(label: &[u8], namespace: Option<&[u8; 32]>) -> Result<Vec<u8>, i32> {
    if label.first() != Some(&BOUND_PREFIX) {
        return Ok(label.to_vec());
    }
    let Some(namespace) = namespace else {
        return Err(EPERM);
    };
    if label.len() > BOUND_LABEL_MAX {
        return Err(EINVAL);
    }
    let mut out = Vec::with_capacity(label.len() + namespace.len());
    out.extend_from_slice(label);
    out.extend_from_slice(namespace);
    Ok(out)
}
