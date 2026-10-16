// NONOS Operating System (AGPL-3.0-or-later)
//! Host-runnable proofs for HID over SPI: the protocol core, and
//! driver.spi_hid0's reading of the firmware's serial-bus record and its
//! shaping of a report for the shared HID poll.

pub mod acpi;
#[path = "../../capsule_driver_spi_hid/src/shape.rs"]
pub mod shape;

#[cfg(test)]
mod frame_tests;
#[cfg(test)]
mod input_tests;
#[cfg(test)]
mod pick_tests;
#[cfg(test)]
mod record;
#[cfg(test)]
mod serial_tests;
#[cfg(test)]
mod shape_tests;
