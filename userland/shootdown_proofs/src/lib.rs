// NONOS Operating System (AGPL-3.0-or-later)
//! The x86_64 TLB shootdown protocol, searched over every interleaving of
//! one round with the CPUs it must reach and the APs that come up during it.

pub mod model;
pub mod search;

#[cfg(test)]
mod tests;
