// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/memory/iommu/device.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/memory/iommu/device.rs"]
pub mod iommu_device;

// The kernel made `DeviceAddress` an enum, a PCI requester in a segment or an
// SMMU StreamID, and removed `new(raw)`. A raw `u32` no longer names a PCI
// requester; the one constructor that still takes a raw `u32` is `stream`, the
// role `new(raw)` had for a non-PCI identifier. The wrapper keeps the name
// `crates.json` lists and forwards to `stream` unchanged.
pub fn deviceaddress_new(raw: u32) -> iommu_device::DeviceAddress {
    iommu_device::DeviceAddress::stream(raw)
}

pub fn deviceaddress_pci(bus: u8, device: u8, function: u8) -> iommu_device::DeviceAddress {
    iommu_device::DeviceAddress::pci(bus, device, function)
}

// The kernel removed `as_u32`. The raw PCI requester id is now `rid`, which is
// `None` for a StreamID, so the wrapper's return type changed from `u32` to
// `Option<u16>`. It keeps the name `crates.json` lists and forwards unchanged.
pub fn deviceaddress_as_u32(this: iommu_device::DeviceAddress) -> Option<u16> {
    this.rid()
}

// The kernel folded `pci_bus`, `pci_device` and `pci_function` into one `bdf`,
// which returns the triple, or `None` for a StreamID. Each of the three
// wrappers keeps the name `crates.json` lists and forwards to `bdf` unchanged,
// so their return type changed from `u8` to `Option<(u8, u8, u8)>`; picking one
// field out of the triple here would be logic the kernel does not have.
pub fn deviceaddress_pci_bus(this: iommu_device::DeviceAddress) -> Option<(u8, u8, u8)> {
    this.bdf()
}

pub fn deviceaddress_pci_device(this: iommu_device::DeviceAddress) -> Option<(u8, u8, u8)> {
    this.bdf()
}

pub fn deviceaddress_pci_function(this: iommu_device::DeviceAddress) -> Option<(u8, u8, u8)> {
    this.bdf()
}
