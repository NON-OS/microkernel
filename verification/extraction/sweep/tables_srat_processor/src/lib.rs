// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/x86_64/acpi/tables/srat_processor.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/x86_64/acpi/tables/srat_processor.rs"]
pub mod srat_processor;

pub fn sratprocessoraffinity_proximity_domain(this: srat_processor::SratProcessorAffinity) -> u32 {
    this.proximity_domain()
}

pub fn sratprocessoraffinity_is_enabled(this: srat_processor::SratProcessorAffinity) -> bool {
    this.is_enabled()
}

pub fn sratx2apicaffinity_is_enabled(this: srat_processor::SratX2ApicAffinity) -> bool {
    this.is_enabled()
}

