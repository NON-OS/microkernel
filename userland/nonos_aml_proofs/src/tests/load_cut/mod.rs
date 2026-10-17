// NONOS Operating System (AGPL-3.0-or-later)
//! A table load cut at every step by the budget (ACPI 6.5, 19.6.74 and
//! 19.6.75 for Load and LoadTable, 6.5.4 for _REG; recalled, unverified):
//! a later evaluation sees the namespace as if the load never began, or as
//! if it finished; never a table half in. A scope whose _REG the cut kept
//! from finishing is not marked told, and is told at the next load.

mod fixture;
mod gen;
mod snap;
mod sweep_tests;
