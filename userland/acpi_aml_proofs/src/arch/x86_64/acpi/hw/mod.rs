// NONOS Operating System (AGPL-3.0-or-later)
//! The pure ACPI fixed-hardware modules. `port_bus` (real ports and MMIO) is
//! left out; the tests drive these through a recording bus instead.
#[path = "../../../../../../../src/arch/x86_64/acpi/hw/gas.rs"]
pub mod gas;
#[path = "../../../../../../../src/arch/x86_64/acpi/hw/fadt_decode.rs"]
pub mod fadt_decode;
#[path = "../../../../../../../src/arch/x86_64/acpi/hw/sleep.rs"]
pub mod sleep;
#[path = "../../../../../../../src/arch/x86_64/acpi/hw/reset.rs"]
pub mod reset;
#[path = "../../../../../../../src/arch/x86_64/acpi/hw/madt_cpu.rs"]
pub mod madt_cpu;
#[path = "../../../../../../../src/arch/x86_64/acpi/hw/power_button.rs"]
pub mod power_button;
