// NONOS Operating System (AGPL-3.0-or-later)
//! Host proofs for the aarch64 IORT reader. `arch` mirrors the kernel's
//! module paths, holding the reader and only the record types it fills, so
//! the `crate::arch::...` paths inside the mounted files resolve.

#[cfg(test)]
mod arch;

#[cfg(test)]
mod table;
#[cfg(test)]
mod table_nodes;

#[cfg(test)]
mod compose_tests;
#[cfg(test)]
mod hostile_tests;
#[cfg(test)]
mod limit_tests;
#[cfg(test)]
mod records_tests;
#[cfg(test)]
mod refusal_tests;
