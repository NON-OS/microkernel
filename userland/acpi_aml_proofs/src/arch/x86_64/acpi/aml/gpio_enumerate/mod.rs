// NONOS Operating System (AGPL-3.0-or-later)
// The GPIO controller enumerator's pure parts; `enumerate` walks the live
// tables and stays out. The proofs sit as a child so they reach the
// pub(super) parsers exactly as the enumerator does.
#[path = "../../../../../../../../src/arch/x86_64/acpi/aml/gpio_enumerate/crs.rs"]
mod crs;
#[path = "../../../../../../../../src/arch/x86_64/acpi/aml/gpio_enumerate/hid_match.rs"]
mod hid_match;
#[path = "../../../../../../../../src/arch/x86_64/acpi/aml/gpio_enumerate/sideband.rs"]
mod sideband;
#[path = "../../../../../../../../src/arch/x86_64/acpi/aml/gpio_enumerate/uid.rs"]
mod uid;

#[cfg(test)]
#[path = "../../../../../gpio_enum_tests/mod.rs"]
mod tests;
