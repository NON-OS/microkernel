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

//! What a proof is kept under (attest_seen_rule.rs): a digest of every byte
//! the kernel's answer depends on. Pure.

/// Sets these keys apart from every other BLAKE3 this capsule takes. Each
/// field after it is preceded by its length, so no two sets of fields share a
/// key.
const DOMAIN: &[u8] = b"nonos.linux.attest.seen.v1";

/// The key of a proof of `image` by `trailer`, with the publisher's
/// certificate and manifest when it has them.
pub fn key(image: &[u8], trailer: &[u8], publisher: Option<(&[u8], &[u8])>) -> [u8; 32] {
    let mut h = blake3::Hasher::new();
    h.update(DOMAIN);
    let (cert, manifest): (&[u8], &[u8]) = publisher.unwrap_or((&[], &[]));
    let kind: &[u8] = if publisher.is_some() { b"publisher" } else { b"local" };
    for field in [kind, image, trailer, cert, manifest] {
        h.update(&(field.len() as u64).to_le_bytes());
        h.update(field);
    }
    *h.finalize().as_bytes()
}
