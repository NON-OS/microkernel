// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

#[cfg(feature = "nonos-capsule-driver-rtl8822ce")]
pub(super) const DRIVER_RTL8822CE_ELF: &[u8] = include_bytes!(concat!(
    "../../../userland/capsule_driver_rtl8822ce/target/",
    env!("NONOS_USER_TARGET"),
    "/release/driver_rtl8822ce"
));

#[cfg(feature = "nonos-capsule-driver-rtl8822ce")]
pub(super) const DRIVER_RTL8822CE_NONOS_ID_CERT_BYTES: &[u8] =
    crate::baked_trust!("capsules/driver_rtl8822ce.nonos_id_cert.bin");

#[cfg(feature = "nonos-capsule-driver-rtl8822ce")]
pub(super) const DRIVER_RTL8822CE_MANIFEST_BYTES: &[u8] =
    crate::baked_trust!("capsules/driver_rtl8822ce.manifest.bin");

#[cfg(feature = "nonos-capsule-driver-rtl8822ce")]
pub(super) const DRIVER_RTL8822CE_ATTESTATION_BYTES: &[u8] =
    crate::baked_trust!("capsules/driver_rtl8822ce.zk_trailer.bin");

// A capsule built in with an empty file never compiles (B-02).
#[cfg(feature = "nonos-capsule-driver-rtl8822ce")]
const _: () = assert!(
    !DRIVER_RTL8822CE_ELF.is_empty()
        && !DRIVER_RTL8822CE_NONOS_ID_CERT_BYTES.is_empty()
        && !DRIVER_RTL8822CE_MANIFEST_BYTES.is_empty()
        && !DRIVER_RTL8822CE_ATTESTATION_BYTES.is_empty(),
    "rtl8822ce_capsule built in with an empty image"
);

#[cfg(not(feature = "nonos-capsule-driver-rtl8822ce"))]
pub(super) const DRIVER_RTL8822CE_ELF: &[u8] = &[];

#[cfg(not(feature = "nonos-capsule-driver-rtl8822ce"))]
pub(super) const DRIVER_RTL8822CE_NONOS_ID_CERT_BYTES: &[u8] = &[];

#[cfg(not(feature = "nonos-capsule-driver-rtl8822ce"))]
pub(super) const DRIVER_RTL8822CE_MANIFEST_BYTES: &[u8] = &[];

#[cfg(not(feature = "nonos-capsule-driver-rtl8822ce"))]
pub(super) const DRIVER_RTL8822CE_ATTESTATION_BYTES: &[u8] = &[];
