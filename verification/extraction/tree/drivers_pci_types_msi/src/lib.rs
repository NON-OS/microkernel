// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/drivers/pci/types/msi.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod drivers;

pub fn msiinfo_max_vectors(this: crate::drivers::pci::types::msi::MsiInfo) -> u8 {
    this.max_vectors()
}

pub fn msiinfo_allocated_vectors(this: crate::drivers::pci::types::msi::MsiInfo) -> u8 {
    this.allocated_vectors()
}

pub fn msixinfo_vector_count(this: crate::drivers::pci::types::msi::MsixInfo) -> u16 {
    this.vector_count()
}

pub fn msimessage_new(vector: u8, dest_id: u8, edge_trigger: bool, level_assert: bool) -> crate::drivers::pci::types::msi::MsiMessage {
    crate::drivers::pci::types::msi::MsiMessage::new(vector, dest_id, edge_trigger, level_assert)
}

pub fn msimessage_for_local_apic(vector: u8, dest_apic_id: u8) -> crate::drivers::pci::types::msi::MsiMessage {
    crate::drivers::pci::types::msi::MsiMessage::for_local_apic(vector, dest_apic_id)
}

