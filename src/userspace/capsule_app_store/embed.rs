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

// Build-time embed of the marketplace window.

#[cfg(feature = "nonos-capsule-app-store")]
pub(crate) const APP_STORE_ELF: &[u8] =
    include_bytes!(concat!(
    "../../../userland/capsule_app_store/target/",
    env!("NONOS_USER_TARGET"),
    "/release/app_store"
));

#[cfg(feature = "nonos-capsule-app-store")]
pub(crate) const APP_STORE_NONOS_ID_CERT_BYTES: &[u8] =
    include_bytes!("../../../nonos-data/trust/capsules/app_store.nonos_id_cert.bin");

#[cfg(feature = "nonos-capsule-app-store")]
pub(crate) const APP_STORE_MANIFEST_BYTES: &[u8] =
    include_bytes!("../../../nonos-data/trust/capsules/app_store.manifest.bin");

#[cfg(feature = "nonos-capsule-app-store")]
pub(crate) const APP_STORE_ATTESTATION_BYTES: &[u8] =
    include_bytes!("../../../nonos-data/trust/capsules/app_store.zk_trailer.bin");

#[cfg(not(feature = "nonos-capsule-app-store"))]
pub(crate) const APP_STORE_ELF: &[u8] = &[];

#[cfg(not(feature = "nonos-capsule-app-store"))]
pub(crate) const APP_STORE_NONOS_ID_CERT_BYTES: &[u8] = &[];

#[cfg(not(feature = "nonos-capsule-app-store"))]
pub(crate) const APP_STORE_MANIFEST_BYTES: &[u8] = &[];

#[cfg(not(feature = "nonos-capsule-app-store"))]
pub(crate) const APP_STORE_ATTESTATION_BYTES: &[u8] = &[];
