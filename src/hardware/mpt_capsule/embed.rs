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

#[cfg(feature = "nonos-capsule-driver-mpt")]
pub(super) const DRIVER_MPT_ELF: &[u8] = include_bytes!(concat!(
    "../../../userland/capsule_driver_mpt/target/",
    env!("NONOS_USER_TARGET"),
    "/release/driver_mpt"
));

#[cfg(feature = "nonos-capsule-driver-mpt")]
pub(super) const DRIVER_MPT_NONOS_ID_CERT_BYTES: &[u8] =
    crate::baked_trust!("capsules/driver_mpt.nonos_id_cert.bin");

#[cfg(feature = "nonos-capsule-driver-mpt")]
pub(super) const DRIVER_MPT_MANIFEST_BYTES: &[u8] =
    crate::baked_trust!("capsules/driver_mpt.manifest.bin");

#[cfg(feature = "nonos-capsule-driver-mpt")]
pub(super) const DRIVER_MPT_ATTESTATION_BYTES: &[u8] =
    crate::baked_trust!("capsules/driver_mpt.zk_trailer.bin");

// A capsule built in with an empty file never compiles (B-02).
#[cfg(feature = "nonos-capsule-driver-mpt")]
const _: () = assert!(
    !DRIVER_MPT_ELF.is_empty()
        && !DRIVER_MPT_NONOS_ID_CERT_BYTES.is_empty()
        && !DRIVER_MPT_MANIFEST_BYTES.is_empty()
        && !DRIVER_MPT_ATTESTATION_BYTES.is_empty(),
    "mpt_capsule built in with an empty image"
);

#[cfg(not(feature = "nonos-capsule-driver-mpt"))]
pub(super) const DRIVER_MPT_ELF: &[u8] = &[];

#[cfg(not(feature = "nonos-capsule-driver-mpt"))]
pub(super) const DRIVER_MPT_NONOS_ID_CERT_BYTES: &[u8] = &[];

#[cfg(not(feature = "nonos-capsule-driver-mpt"))]
pub(super) const DRIVER_MPT_MANIFEST_BYTES: &[u8] = &[];

#[cfg(not(feature = "nonos-capsule-driver-mpt"))]
pub(super) const DRIVER_MPT_ATTESTATION_BYTES: &[u8] = &[];
