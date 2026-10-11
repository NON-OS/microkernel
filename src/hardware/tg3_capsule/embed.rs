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
//! Build-time embed of the tg3 driver capsule. The kernel feature
//! `nonos-capsule-driver-tg3` selects whether the signed bytes are pulled
//! into the image or replaced by empty slices.

#[cfg(feature = "nonos-capsule-driver-tg3")]
pub(super) const DRIVER_TG3_ELF: &[u8] = include_bytes!(concat!(
    "../../../userland/capsule_driver_tg3/target/",
    env!("NONOS_USER_TARGET"),
    "/release/driver_tg3"
));

#[cfg(feature = "nonos-capsule-driver-tg3")]
pub(super) const DRIVER_TG3_NONOS_ID_CERT_BYTES: &[u8] =
    include_bytes!("../../../nonos-data/trust/capsules/driver_tg3.nonos_id_cert.bin");

#[cfg(feature = "nonos-capsule-driver-tg3")]
pub(super) const DRIVER_TG3_MANIFEST_BYTES: &[u8] =
    include_bytes!("../../../nonos-data/trust/capsules/driver_tg3.manifest.bin");

#[cfg(feature = "nonos-capsule-driver-tg3")]
pub(super) const DRIVER_TG3_ATTESTATION_BYTES: &[u8] =
    include_bytes!("../../../nonos-data/trust/capsules/driver_tg3.zk_trailer.bin");

#[cfg(not(feature = "nonos-capsule-driver-tg3"))]
pub(super) const DRIVER_TG3_ELF: &[u8] = &[];

#[cfg(not(feature = "nonos-capsule-driver-tg3"))]
pub(super) const DRIVER_TG3_NONOS_ID_CERT_BYTES: &[u8] = &[];

#[cfg(not(feature = "nonos-capsule-driver-tg3"))]
pub(super) const DRIVER_TG3_MANIFEST_BYTES: &[u8] = &[];

#[cfg(not(feature = "nonos-capsule-driver-tg3"))]
pub(super) const DRIVER_TG3_ATTESTATION_BYTES: &[u8] = &[];
