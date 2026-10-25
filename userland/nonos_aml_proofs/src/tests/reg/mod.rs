// NONOS Operating System (AGPL-3.0-or-later)
//! _REG across spaces, devices and tables (ACPI 6.5, 6.5.4; recalled,
//! unverified): _REG(space, 1) tells a scope that the handler for a space
//! its regions use is there. The specification gives no order between
//! devices; the one proved is this interpreter's: per connected space, one
//! call per scope holding a region of it, in the order the regions were
//! defined, never twice for one scope and space, and again for the regions
//! a table brings once their space is already connected.

mod before_tests;
mod fixture;
mod gen;
mod model;
mod reload_tests;
mod sweep_tests;
