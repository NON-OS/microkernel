// NONOS Operating System (AGPL-3.0-or-later)
//! What the battery call asks of the rest of the kernel, modelled as the
//! machine where nothing reports: the ACPI capsule has sent no report, so the
//! fresh-report path is the kernel's `report_absent.rs`, and the firmware's
//! answer on whether its tables declare a battery is one the kernel does not
//! have yet. The call's branches on both are the kernel's own code, included
//! unchanged.

/// The kernel's `acpi_only!` on a platform with no ACPI report: the plain form
/// emits nothing and the `else` form keeps its items, so the call mounts
/// `report_absent.rs`.
macro_rules! acpi_only {
    (else $($item:item)*) => { $($item)* };
    ($($item:item)*) => {};
}
pub(crate) use acpi_only;

/// Whether the firmware's tables declare a battery: not known yet.
pub fn acpi_tables_declare_battery() -> Option<bool> {
    None
}
