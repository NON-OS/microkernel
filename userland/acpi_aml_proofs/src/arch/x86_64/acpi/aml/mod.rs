// NONOS Operating System (AGPL-3.0-or-later)
#[path = "../../../../../../../src/arch/x86_64/acpi/aml/scan.rs"]
pub mod scan;
#[path = "../../../../../../../src/arch/x86_64/acpi/aml/crs.rs"]
pub mod crs;
#[path = "../../../../../../../src/arch/x86_64/acpi/aml/types/mod.rs"]
pub mod types;
#[path = "../../../../../../../src/arch/x86_64/acpi/aml/sleep_obj.rs"]
pub mod sleep_obj;
#[path = "../../../../../../../src/arch/x86_64/acpi/aml/power_devices.rs"]
pub mod power_devices;

pub mod controller;
// Its parsers are pub(super), reached only by the proofs nested inside it.
#[cfg(test)]
pub mod gpio_enumerate;
