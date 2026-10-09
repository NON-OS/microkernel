// NONOS Operating System (AGPL-3.0-or-later)

pub mod aml;
pub mod data;
pub mod hw;

#[path = "../../../../../../../../src/arch/x86_64/acpi/tables/mod.rs"]
pub mod tables;

pub use data::{AcpiData, InterruptOverride, IoApicInfo, NmiConfig, NumaMemoryRegion, PcieSegment, ProcessorInfo};
pub use tables::{fadt_flags, madt_flags, AddressSpace, Fadt, GenericAddress, Hpet, Madt, MadtEntryHeader, MadtEntryType, MadtInterruptOverride, MadtIoApic, MadtLocalApic, MadtLocalApicNmi, MadtLocalApicOverride, MadtLocalX2Apic, MadtLocalX2ApicNmi, MadtNmiSource, Mcfg, McfgEntry, PmProfile, Rsdp, RsdpExtended, SdtHeader, Slit, Srat, SratEntryType, SratMemoryAffinity, SratProcessorAffinity, SratX2ApicAffinity, RSDP_ALIGNMENT, RSDP_SIGNATURE, SIG_FADT, SIG_HPET, SIG_MADT, SIG_MCFG, SIG_RSDT, SIG_SLIT, SIG_SRAT, SIG_XSDT};
