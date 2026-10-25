// NONOS Operating System (AGPL-3.0-or-later)
//! Host proofs for the aarch64 GIC's pure parts. `gic` mirrors the kernel's
//! module layout so the `super::` paths inside the mounted files resolve.

pub mod gic;

#[cfg(test)]
mod encode_tests;
#[cfg(test)]
mod intid_tests;
#[cfg(test)]
mod irq_handlers_tests;
#[cfg(test)]
mod its_cmd_tests;
#[cfg(test)]
mod its_table_tests;
#[cfg(test)]
mod its_driver;
#[cfg(test)]
mod its_model;
#[cfg(test)]
mod its_queue_close_tests;
#[cfg(test)]
mod its_queue_tests;
#[cfg(test)]
mod select_tests;
