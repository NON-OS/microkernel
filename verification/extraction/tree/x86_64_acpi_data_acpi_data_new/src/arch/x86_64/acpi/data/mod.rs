// NONOS Operating System (AGPL-3.0-or-later)

#[path = "../../../../../../../../../src/arch/x86_64/acpi/data/acpi_data_new.rs"]
pub mod acpi_data_new;

#[path = "../../../../../../../../../src/arch/x86_64/acpi/data/acpi_data_struct.rs"]
pub mod acpi_data_struct;

#[path = "../../../../../../../../../src/arch/x86_64/acpi/data/interrupt.rs"]
pub mod interrupt;

#[path = "../../../../../../../../../src/arch/x86_64/acpi/data/ioapic.rs"]
pub mod ioapic;

#[path = "../../../../../../../../../src/arch/x86_64/acpi/data/numa.rs"]
pub mod numa;

#[path = "../../../../../../../../../src/arch/x86_64/acpi/data/pcie.rs"]
pub mod pcie;

#[path = "../../../../../../../../../src/arch/x86_64/acpi/data/processor.rs"]
pub mod processor;

pub use acpi_data_struct::AcpiData;
pub use interrupt::{InterruptOverride, NmiConfig};
pub use ioapic::IoApicInfo;
pub use numa::NumaMemoryRegion;
pub use pcie::PcieSegment;
pub use processor::ProcessorInfo;
