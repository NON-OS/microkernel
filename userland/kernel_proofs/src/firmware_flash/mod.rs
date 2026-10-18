// NONOS Operating System (AGPL-3.0-or-later)
//! The firmware's flash is never claimed or mapped: the shipped guard, and
//! the claim and map paths that consult it.

mod edge_tests;
#[path = "../../../../src/hardware/broker/firmware_flash.rs"]
mod guard;
mod guard_tests;
