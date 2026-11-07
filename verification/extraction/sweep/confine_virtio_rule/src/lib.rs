// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/hardware/broker/confine/virtio_rule.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/hardware/broker/confine/virtio_rule.rs"]
pub mod virtio_rule;

pub fn is_modern_virtio(vendor: u16, device: u16) -> bool {
    virtio_rule::is_modern_virtio(vendor, device)
}

