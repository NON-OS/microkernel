// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/hardware/broker/acpi/sleep_request.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/hardware/broker/acpi/sleep_request.rs"]
pub mod sleep_request;

pub fn valid(state: u64, a: u64, b: u64) -> bool {
    sleep_request::valid(state, a, b)
}

