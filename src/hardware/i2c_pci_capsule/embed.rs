// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

#[cfg(feature = "nonos-capsule-driver-i2c-pci")]
pub(super) const DRIVER_I2C_PCI_ELF: &[u8] = include_bytes!(concat!(
    "../../../userland/capsule_driver_i2c_pci/target/",
    env!("NONOS_USER_TARGET"),
    "/release/driver_i2c_pci"
));

#[cfg(feature = "nonos-capsule-driver-i2c-pci")]
pub(super) const DRIVER_I2C_PCI_NONOS_ID_CERT_BYTES: &[u8] =
    crate::baked_trust!("capsules/driver_i2c_pci.nonos_id_cert.bin");

#[cfg(feature = "nonos-capsule-driver-i2c-pci")]
pub(super) const DRIVER_I2C_PCI_MANIFEST_BYTES: &[u8] =
    crate::baked_trust!("capsules/driver_i2c_pci.manifest.bin");

#[cfg(feature = "nonos-capsule-driver-i2c-pci")]
pub(super) const DRIVER_I2C_PCI_ATTESTATION_BYTES: &[u8] =
    crate::baked_trust!("capsules/driver_i2c_pci.zk_trailer.bin");

// A capsule built in with an empty file never compiles (B-02).
#[cfg(feature = "nonos-capsule-driver-i2c-pci")]
const _: () = assert!(
    !DRIVER_I2C_PCI_ELF.is_empty()
        && !DRIVER_I2C_PCI_NONOS_ID_CERT_BYTES.is_empty()
        && !DRIVER_I2C_PCI_MANIFEST_BYTES.is_empty()
        && !DRIVER_I2C_PCI_ATTESTATION_BYTES.is_empty(),
    "i2c_pci_capsule built in with an empty image"
);

#[cfg(not(feature = "nonos-capsule-driver-i2c-pci"))]
pub(super) const DRIVER_I2C_PCI_ELF: &[u8] = &[];

#[cfg(not(feature = "nonos-capsule-driver-i2c-pci"))]
pub(super) const DRIVER_I2C_PCI_NONOS_ID_CERT_BYTES: &[u8] = &[];

#[cfg(not(feature = "nonos-capsule-driver-i2c-pci"))]
pub(super) const DRIVER_I2C_PCI_MANIFEST_BYTES: &[u8] = &[];

#[cfg(not(feature = "nonos-capsule-driver-i2c-pci"))]
pub(super) const DRIVER_I2C_PCI_ATTESTATION_BYTES: &[u8] = &[];
