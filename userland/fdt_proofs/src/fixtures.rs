// NONOS Operating System (AGPL-3.0-or-later)
//! The blobs under `fixtures/`, compiled from the `.dts` beside each one.

pub const VIRT_GICV3: &[u8] = include_bytes!("../fixtures/virt-gicv3.dtb");
pub const VIRT_GICV2: &[u8] = include_bytes!("../fixtures/virt-gicv2.dtb");
pub const VIRT_SMMUV3: &[u8] = include_bytes!("../fixtures/virt-smmuv3.dtb");
pub const RPI4B: &[u8] = include_bytes!("../fixtures/rpi4b.dtb");

pub const ALL: &[&[u8]] = &[VIRT_GICV3, VIRT_GICV2, VIRT_SMMUV3, RPI4B];
