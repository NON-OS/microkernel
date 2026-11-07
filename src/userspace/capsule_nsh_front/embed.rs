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

//! The nsh.front image and its trust artifacts, baked into the kernel when
//! the capsule is built in. The id cert, manifest and attestation trailer
//! are made by the development image's seal with throwaway keys; a sealed
//! release makes them at the release signing ceremony.

#[cfg(feature = "nonos-capsule-nsh-front")]
pub(crate) const NSH_FRONT_ELF: &[u8] = include_bytes!(concat!(
    "../../../userland/capsule_nsh_front/target/",
    env!("NONOS_USER_TARGET"),
    "/release/nsh_front"
));

#[cfg(feature = "nonos-capsule-nsh-front")]
pub(crate) const NSH_FRONT_NONOS_ID_CERT_BYTES: &[u8] =
    crate::baked_trust!("capsules/nsh_front.nonos_id_cert.bin");

#[cfg(feature = "nonos-capsule-nsh-front")]
pub(crate) const NSH_FRONT_MANIFEST_BYTES: &[u8] =
    crate::baked_trust!("capsules/nsh_front.manifest.bin");

#[cfg(feature = "nonos-capsule-nsh-front")]
pub(crate) const NSH_FRONT_ATTESTATION_BYTES: &[u8] =
    crate::baked_trust!("capsules/nsh_front.zk_trailer.bin");

// A capsule built in with an empty file never compiles (B-02).
#[cfg(feature = "nonos-capsule-nsh-front")]
const _: () = assert!(
    !NSH_FRONT_ELF.is_empty()
        && !NSH_FRONT_NONOS_ID_CERT_BYTES.is_empty()
        && !NSH_FRONT_MANIFEST_BYTES.is_empty()
        && !NSH_FRONT_ATTESTATION_BYTES.is_empty(),
    "capsule_nsh_front built in with an empty image"
);

#[cfg(not(feature = "nonos-capsule-nsh-front"))]
pub(crate) const NSH_FRONT_ELF: &[u8] = &[];

#[cfg(not(feature = "nonos-capsule-nsh-front"))]
pub(crate) const NSH_FRONT_NONOS_ID_CERT_BYTES: &[u8] = &[];

#[cfg(not(feature = "nonos-capsule-nsh-front"))]
pub(crate) const NSH_FRONT_MANIFEST_BYTES: &[u8] = &[];

#[cfg(not(feature = "nonos-capsule-nsh-front"))]
pub(crate) const NSH_FRONT_ATTESTATION_BYTES: &[u8] = &[];
