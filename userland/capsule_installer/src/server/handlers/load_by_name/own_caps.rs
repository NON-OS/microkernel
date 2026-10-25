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

//! The capabilities a manifest names for its capsule, required and optional,
//! read from the head of `<name>.manifest.bin`. A sender never picks what an
//! app it starts holds: the load asks for exactly this set, and the kernel
//! refuses a request beyond it (`from_vfs/load/asked.rs`). Unverified bytes,
//! which is safe here: the kernel checks the manifest's signature before the
//! spawn, and a request beyond the signed manifest is refused.

/* The head as `security/capsule_manifest/decode/header.rs` reads it. */
const SCHEMA: u16 = 3;
const CERT_ID: usize = 32;
const PAYLOAD_HASH: usize = 32;

/// What a load asks for: the sender's set when it names one, else the
/// manifest's own. None when the manifest's head cannot be read.
pub(super) fn asked(sent: u64, manifest: &[u8]) -> Option<u64> {
    if sent != 0 {
        return Some(sent);
    }
    own_caps(manifest)
}

/// `required | optional` from the manifest's head, or None for one this
/// schema does not read.
pub(super) fn own_caps(m: &[u8]) -> Option<u64> {
    let mut at = 0usize;
    let mut take = |n: usize| -> Option<&[u8]> {
        let s = m.get(at..at.checked_add(n)?)?;
        at += n;
        Some(s)
    };
    if u16::from_be_bytes(take(2)?.try_into().ok()?) != SCHEMA {
        return None;
    }
    take(CERT_ID)?;
    let namespace = take(1)?[0] as usize;
    take(namespace)?;
    take(12)?;
    let target = take(1)?[0] as usize;
    take(target)?;
    take(PAYLOAD_HASH)?;
    let required = u64::from_be_bytes(take(8)?.try_into().ok()?);
    let optional = u64::from_be_bytes(take(8)?.try_into().ok()?);
    Some(required | optional)
}
