// NONOS Operating System (AGPL-3.0-or-later)
// The kernel's own gate: on this x86_64 host, as in the kernel there, the
// ACPI broker's writers are compiled and these proofs hold them.
#[cfg(test)]
#[path = "../../../../src/arch/acpi_capsule/gate.rs"]
mod acpi_gate;
#[cfg(test)]
pub(crate) use acpi_gate::acpi_only;
#[cfg(test)]
pub mod fdt;
pub mod paging;
#[cfg(test)]
pub mod x86_64;
