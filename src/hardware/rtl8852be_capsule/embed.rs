// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

#[cfg(feature = "nonos-capsule-driver-rtl8852be")]
pub(super) const DRIVER_RTL8852BE_ELF: &[u8] = include_bytes!(concat!(
    "../../../userland/capsule_driver_rtl8852be/target/",
    env!("NONOS_USER_TARGET"),
    "/release/driver_rtl8852be"
));

#[cfg(feature = "nonos-capsule-driver-rtl8852be")]
pub(super) const DRIVER_RTL8852BE_NONOS_ID_CERT_BYTES: &[u8] =
    crate::baked_trust!("capsules/driver_rtl8852be.nonos_id_cert.bin");

#[cfg(feature = "nonos-capsule-driver-rtl8852be")]
pub(super) const DRIVER_RTL8852BE_MANIFEST_BYTES: &[u8] =
    crate::baked_trust!("capsules/driver_rtl8852be.manifest.bin");

#[cfg(feature = "nonos-capsule-driver-rtl8852be")]
pub(super) const DRIVER_RTL8852BE_ATTESTATION_BYTES: &[u8] =
    crate::baked_trust!("capsules/driver_rtl8852be.zk_trailer.bin");

// A capsule built in with an empty file never compiles (B-02).
#[cfg(feature = "nonos-capsule-driver-rtl8852be")]
const _: () = assert!(
    !DRIVER_RTL8852BE_ELF.is_empty()
        && !DRIVER_RTL8852BE_NONOS_ID_CERT_BYTES.is_empty()
        && !DRIVER_RTL8852BE_MANIFEST_BYTES.is_empty()
        && !DRIVER_RTL8852BE_ATTESTATION_BYTES.is_empty(),
    "rtl8852be_capsule built in with an empty image"
);

#[cfg(not(feature = "nonos-capsule-driver-rtl8852be"))]
pub(super) const DRIVER_RTL8852BE_ELF: &[u8] = &[];

#[cfg(not(feature = "nonos-capsule-driver-rtl8852be"))]
pub(super) const DRIVER_RTL8852BE_NONOS_ID_CERT_BYTES: &[u8] = &[];

#[cfg(not(feature = "nonos-capsule-driver-rtl8852be"))]
pub(super) const DRIVER_RTL8852BE_MANIFEST_BYTES: &[u8] = &[];

#[cfg(not(feature = "nonos-capsule-driver-rtl8852be"))]
pub(super) const DRIVER_RTL8852BE_ATTESTATION_BYTES: &[u8] = &[];
