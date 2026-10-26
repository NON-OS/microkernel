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

//! nonos.prove.fetch's ELF, identity certificate, signed manifest and
//! attestation trailer, baked at build time. Empty when the feature is off,
//! so the spawn refuses and nothing runs.

#[cfg(feature = "nonos-capsule-prove-fetch")]
pub(super) const ELF: &[u8] = include_bytes!(concat!(
    "../../../userland/capsule_prove_fetch/target/",
    env!("NONOS_USER_TARGET"),
    "/release/prove-fetch"
));

#[cfg(feature = "nonos-capsule-prove-fetch")]
pub(super) const CERT: &[u8] = crate::baked_trust!("capsules/prove-fetch.nonos_id_cert.bin");

#[cfg(feature = "nonos-capsule-prove-fetch")]
pub(super) const MANIFEST: &[u8] = crate::baked_trust!("capsules/prove-fetch.manifest.bin");

#[cfg(feature = "nonos-capsule-prove-fetch")]
pub(super) const ATTESTATION: &[u8] = crate::baked_trust!("capsules/prove-fetch.zk_trailer.bin");

// A capsule built in with an empty file never compiles (B-02).
#[cfg(feature = "nonos-capsule-prove-fetch")]
const _: () = assert!(
    !ELF.is_empty() && !CERT.is_empty() && !MANIFEST.is_empty() && !ATTESTATION.is_empty(),
    "capsule_prove_fetch built in with an empty image"
);

#[cfg(not(feature = "nonos-capsule-prove-fetch"))]
pub(super) const ELF: &[u8] = &[];

#[cfg(not(feature = "nonos-capsule-prove-fetch"))]
pub(super) const CERT: &[u8] = &[];

#[cfg(not(feature = "nonos-capsule-prove-fetch"))]
pub(super) const MANIFEST: &[u8] = &[];

#[cfg(not(feature = "nonos-capsule-prove-fetch"))]
pub(super) const ATTESTATION: &[u8] = &[];
