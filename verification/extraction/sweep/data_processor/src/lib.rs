// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/x86_64/acpi/data/processor.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/x86_64/acpi/data/processor.rs"]
pub mod processor;

pub fn processorinfo_new(apic_id: u32, processor_uid: u32, is_x2apic: bool, enabled: bool) -> processor::ProcessorInfo {
    processor::ProcessorInfo::new(apic_id, processor_uid, is_x2apic, enabled)
}

