// NONOS Operating System (AGPL-3.0-or-later)
#[path = "../../../../../../../../src/arch/x86_64/acpi/aml/controller/memory32.rs"]
mod memory32;
#[path = "../../../../../../../../src/arch/x86_64/acpi/aml/controller/interrupt.rs"]
mod interrupt;
#[path = "../../../../../../../../src/arch/x86_64/acpi/aml/controller/hid_match.rs"]
mod hid_match;
#[path = "../../../../../../../../src/arch/x86_64/acpi/aml/controller/apply.rs"]
mod apply;
#[path = "../../../../../../../../src/arch/x86_64/acpi/aml/controller/crs.rs"]
mod crs;
#[path = "../../../../../../../../src/arch/x86_64/acpi/aml/controller/find.rs"]
mod find;

pub use crs::parse_controller_crs;
pub use find::find_i2c_controller_devices;
// The GPIO community enumerator reuses the descriptor decode, as in the kernel.
#[cfg(test)]
pub(super) use memory32::parse_memory32_fixed;
