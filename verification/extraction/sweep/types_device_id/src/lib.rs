// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/drivers/pci/types/device_id.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/drivers/pci/types/device_id.rs"]
pub mod types_device_id;

pub fn deviceid_new(vendor_id: u16, device_id: u16) -> types_device_id::DeviceId {
    types_device_id::DeviceId::new(vendor_id, device_id)
}

pub fn deviceid_matches(this: types_device_id::DeviceId, vendor: u16, device: u16) -> bool {
    this.matches(vendor, device)
}

