// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/memory/iommu/device.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/memory/iommu/device.rs"]
pub mod iommu_device;

pub fn deviceaddress_new(raw: u32) -> iommu_device::DeviceAddress {
    iommu_device::DeviceAddress::new(raw)
}

pub fn deviceaddress_pci(bus: u8, device: u8, function: u8) -> iommu_device::DeviceAddress {
    iommu_device::DeviceAddress::pci(bus, device, function)
}

pub fn deviceaddress_as_u32(this: iommu_device::DeviceAddress) -> u32 {
    this.as_u32()
}

pub fn deviceaddress_pci_bus(this: iommu_device::DeviceAddress) -> u8 {
    this.pci_bus()
}

pub fn deviceaddress_pci_device(this: iommu_device::DeviceAddress) -> u8 {
    this.pci_device()
}

pub fn deviceaddress_pci_function(this: iommu_device::DeviceAddress) -> u8 {
    this.pci_function()
}

