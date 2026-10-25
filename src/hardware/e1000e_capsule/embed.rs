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
//! Build-time embed of the e1000e driver capsule. The kernel feature
//! `nonos-capsule-driver-e1000e` selects whether the signed bytes are pulled
//! into the image or replaced by empty slices.

#[cfg(feature = "nonos-capsule-driver-e1000e")]
pub(super) const DRIVER_E1000E_ELF: &[u8] = include_bytes!(concat!(
    "../../../userland/capsule_driver_e1000e/target/",
    env!("NONOS_USER_TARGET"),
    "/release/driver_e1000e"
));

#[cfg(feature = "nonos-capsule-driver-e1000e")]
pub(super) const DRIVER_E1000E_NONOS_ID_CERT_BYTES: &[u8] =
    include_bytes!("../../../nonos-data/trust/capsules/driver_e1000e.nonos_id_cert.bin");

#[cfg(feature = "nonos-capsule-driver-e1000e")]
pub(super) const DRIVER_E1000E_MANIFEST_BYTES: &[u8] =
    include_bytes!("../../../nonos-data/trust/capsules/driver_e1000e.manifest.bin");

#[cfg(feature = "nonos-capsule-driver-e1000e")]
pub(super) const DRIVER_E1000E_ATTESTATION_BYTES: &[u8] =
    include_bytes!("../../../nonos-data/trust/capsules/driver_e1000e.zk_trailer.bin");

#[cfg(not(feature = "nonos-capsule-driver-e1000e"))]
pub(super) const DRIVER_E1000E_ELF: &[u8] = &[];

#[cfg(not(feature = "nonos-capsule-driver-e1000e"))]
pub(super) const DRIVER_E1000E_NONOS_ID_CERT_BYTES: &[u8] = &[];

#[cfg(not(feature = "nonos-capsule-driver-e1000e"))]
pub(super) const DRIVER_E1000E_MANIFEST_BYTES: &[u8] = &[];

#[cfg(not(feature = "nonos-capsule-driver-e1000e"))]
pub(super) const DRIVER_E1000E_ATTESTATION_BYTES: &[u8] = &[];
