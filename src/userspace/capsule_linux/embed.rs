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

//! The personality's ELF, identity certificate, signed manifest and
//! attestation trailer, baked at build time. Empty when the feature is
//! off, so the spawn plan compiles either way and admits nothing.

#[cfg(feature = "nonos-capsule-linux")]
pub(crate) const LINUX_ELF: &[u8] = include_bytes!(concat!(
    "../../../userland/capsule_linux/target/",
    env!("NONOS_USER_TARGET"),
    "/release/linux"
));

#[cfg(feature = "nonos-capsule-linux")]
pub(crate) const LINUX_NONOS_ID_CERT_BYTES: &[u8] =
    include_bytes!("../../../nonos-data/trust/capsules/linux.nonos_id_cert.bin");

#[cfg(feature = "nonos-capsule-linux")]
pub(crate) const LINUX_MANIFEST_BYTES: &[u8] =
    include_bytes!("../../../nonos-data/trust/capsules/linux.manifest.bin");

#[cfg(feature = "nonos-capsule-linux")]
pub(crate) const LINUX_ATTESTATION_BYTES: &[u8] =
    include_bytes!("../../../nonos-data/trust/capsules/linux.zk_trailer.bin");

#[cfg(not(feature = "nonos-capsule-linux"))]
pub(crate) const LINUX_ELF: &[u8] = &[];
#[cfg(not(feature = "nonos-capsule-linux"))]
pub(crate) const LINUX_NONOS_ID_CERT_BYTES: &[u8] = &[];
#[cfg(not(feature = "nonos-capsule-linux"))]
pub(crate) const LINUX_MANIFEST_BYTES: &[u8] = &[];
#[cfg(not(feature = "nonos-capsule-linux"))]
pub(crate) const LINUX_ATTESTATION_BYTES: &[u8] = &[];
