// NONOS Operating System (AGPL-3.0-or-later)
//! Host proofs for the ACPI AML resource extractor. A directory tree mirroring
//! the kernel's module path lets the included files' absolute `crate::` paths
//! resolve unchanged. The firmware-facing `enumerate` and `tables` modules are
//! omitted; the pure parsers they drive are what these proofs pin.

extern crate alloc;

pub mod arch;

// The SDT entry-count helper is pure, so it is included flat rather than through
// the mirrored module tree.
#[path = "../../../src/arch/x86_64/acpi/tables/sdt/entry_count.rs"]
pub mod sdt_entry_count;

#[cfg(test)]
mod fixtures;
#[cfg(test)]
mod crs_tests;
#[cfg(test)]
mod entry_count_tests;
#[cfg(test)]
mod i2c_hid_enum_tests;
#[cfg(test)]
mod scan_tests;
#[cfg(test)]
mod bus_fake;
#[cfg(test)]
mod gas_tests;
#[cfg(test)]
mod fadt_tests;
#[cfg(test)]
mod sleep_tests;
#[cfg(test)]
mod power_button_tests;
#[cfg(test)]
mod reset_tests;
#[cfg(test)]
mod madt_cpu_tests;
#[cfg(test)]
mod sleep_obj_tests;
#[cfg(test)]
mod power_devices_tests;
