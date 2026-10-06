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

//! The local trailer: magic, the local root, and a keyed tag over the
//! capsule's context. Only the kernel holding the key can make or check one.

extern crate alloc;

use alloc::vec::Vec;

use crate::security::capsule_attest::layout::POLICY_EPOCH;

pub const MAGIC: &[u8; 8] = b"NLOCALK1";
pub(super) const TRAILER_LEN: usize = 8 + 32 + 32;

/* Separates this tag from every other use of the key. */
const TAG_DOMAIN: &[u8] = b"NONOS-LOCAL-SIGN-v1";

/// The context the path leaf binds, laid out the same way: the ELF's hash, the
/// granted capabilities and the epoch, so a tag minted for a capsule holding
/// nothing does not verify for the same bytes installed with more.
pub(super) fn context(digest: &[u8; 32], granted_caps: u64) -> [u8; 48] {
    let mut ctx = [0u8; 48];
    ctx[..32].copy_from_slice(digest);
    ctx[32..40].copy_from_slice(&granted_caps.to_be_bytes());
    ctx[40..48].copy_from_slice(&POLICY_EPOCH.to_be_bytes());
    ctx
}

pub(super) fn tag(key: &[u8; 32], ctx: &[u8; 48]) -> blake3::Hash {
    let mut h = blake3::Hasher::new_keyed(key);
    h.update(TAG_DOMAIN);
    h.update(ctx);
    h.finalize()
}

pub(super) fn encode(root: &[u8; 32], tag: &blake3::Hash) -> Vec<u8> {
    let mut out = Vec::with_capacity(TRAILER_LEN);
    out.extend_from_slice(MAGIC);
    out.extend_from_slice(root);
    out.extend_from_slice(tag.as_bytes());
    out
}

/// The root and tag a trailer carries, or `None` for any other shape.
pub(super) fn decode(trailer: &[u8]) -> Option<([u8; 32], [u8; 32])> {
    if trailer.len() != TRAILER_LEN || !trailer.starts_with(MAGIC) {
        return None;
    }
    let mut root = [0u8; 32];
    let mut tag = [0u8; 32];
    root.copy_from_slice(&trailer[8..40]);
    tag.copy_from_slice(&trailer[40..72]);
    Some((root, tag))
}
