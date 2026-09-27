// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/drivers/pci/types/bridge.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/drivers/pci/types/bridge.rs"]
pub mod bridge;

pub fn bridgeinfo_new() -> bridge::BridgeInfo {
    bridge::BridgeInfo::new()
}

