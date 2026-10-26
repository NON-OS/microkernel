// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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
//! Build-time embed of the igc driver capsule. The kernel feature
//! `nonos-capsule-driver-igc` selects whether the signed bytes are pulled
//! into the image or replaced by empty slices.

#[cfg(feature = "nonos-capsule-driver-igc")]
pub(super) const DRIVER_IGC_ELF: &[u8] = include_bytes!(concat!(
    "../../../userland/capsule_driver_igc/target/",
    env!("NONOS_USER_TARGET"),
    "/release/driver_igc"
));

#[cfg(feature = "nonos-capsule-driver-igc")]
pub(super) const DRIVER_IGC_NONOS_ID_CERT_BYTES: &[u8] =
    crate::baked_trust!("capsules/driver_igc.nonos_id_cert.bin");

#[cfg(feature = "nonos-capsule-driver-igc")]
pub(super) const DRIVER_IGC_MANIFEST_BYTES: &[u8] =
    crate::baked_trust!("capsules/driver_igc.manifest.bin");

#[cfg(feature = "nonos-capsule-driver-igc")]
pub(super) const DRIVER_IGC_ATTESTATION_BYTES: &[u8] =
    crate::baked_trust!("capsules/driver_igc.zk_trailer.bin");

// A capsule built in with an empty file never compiles (B-02).
#[cfg(feature = "nonos-capsule-driver-igc")]
const _: () = assert!(
    !DRIVER_IGC_ELF.is_empty()
        && !DRIVER_IGC_NONOS_ID_CERT_BYTES.is_empty()
        && !DRIVER_IGC_MANIFEST_BYTES.is_empty()
        && !DRIVER_IGC_ATTESTATION_BYTES.is_empty(),
    "igc_capsule built in with an empty image"
);

#[cfg(not(feature = "nonos-capsule-driver-igc"))]
pub(super) const DRIVER_IGC_ELF: &[u8] = &[];

#[cfg(not(feature = "nonos-capsule-driver-igc"))]
pub(super) const DRIVER_IGC_NONOS_ID_CERT_BYTES: &[u8] = &[];

#[cfg(not(feature = "nonos-capsule-driver-igc"))]
pub(super) const DRIVER_IGC_MANIFEST_BYTES: &[u8] = &[];

#[cfg(not(feature = "nonos-capsule-driver-igc"))]
pub(super) const DRIVER_IGC_ATTESTATION_BYTES: &[u8] = &[];
