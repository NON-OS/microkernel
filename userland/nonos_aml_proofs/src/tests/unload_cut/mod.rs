// NONOS Operating System (AGPL-3.0-or-later)
//! Unload (ACPI 6.5, 19.6.146; recalled, unverified) of a table whose
//! objects are in use, cut at every step: the table's objects are all
//! there or all gone; nothing the runtime keeps names a freed object; a
//! mutex taken from the table is given back; a reference kept to one of
//! its objects reads as stale, even once a reload reuses the slot; and a
//! reloaded table's mutex and event start fresh.

mod fixture;
mod gen;
mod sweep_tests;
