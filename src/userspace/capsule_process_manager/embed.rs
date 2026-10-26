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

#[cfg(feature = "nonos-capsule-process-manager")]
pub(crate) const PROCESS_MANAGER_ELF: &[u8] = include_bytes!(concat!(
    "../../../userland/capsule_process_manager/target/",
    env!("NONOS_USER_TARGET"),
    "/release/process_manager"
));

#[cfg(feature = "nonos-capsule-process-manager")]
pub(crate) const PROCESS_MANAGER_NONOS_ID_CERT_BYTES: &[u8] =
    crate::baked_trust!("capsules/process_manager.nonos_id_cert.bin");

#[cfg(feature = "nonos-capsule-process-manager")]
pub(crate) const PROCESS_MANAGER_MANIFEST_BYTES: &[u8] =
    crate::baked_trust!("capsules/process_manager.manifest.bin");

#[cfg(feature = "nonos-capsule-process-manager")]
pub(crate) const PROCESS_MANAGER_ATTESTATION_BYTES: &[u8] =
    crate::baked_trust!("capsules/process_manager.zk_trailer.bin");

// A capsule built in with an empty file never compiles (B-02).
#[cfg(feature = "nonos-capsule-process-manager")]
const _: () = assert!(
    !PROCESS_MANAGER_ELF.is_empty()
        && !PROCESS_MANAGER_NONOS_ID_CERT_BYTES.is_empty()
        && !PROCESS_MANAGER_MANIFEST_BYTES.is_empty()
        && !PROCESS_MANAGER_ATTESTATION_BYTES.is_empty(),
    "capsule_process_manager built in with an empty image"
);

#[cfg(not(feature = "nonos-capsule-process-manager"))]
pub(crate) const PROCESS_MANAGER_ELF: &[u8] = &[];

#[cfg(not(feature = "nonos-capsule-process-manager"))]
pub(crate) const PROCESS_MANAGER_NONOS_ID_CERT_BYTES: &[u8] = &[];

#[cfg(not(feature = "nonos-capsule-process-manager"))]
pub(crate) const PROCESS_MANAGER_MANIFEST_BYTES: &[u8] = &[];

#[cfg(not(feature = "nonos-capsule-process-manager"))]
pub(crate) const PROCESS_MANAGER_ATTESTATION_BYTES: &[u8] = &[];
