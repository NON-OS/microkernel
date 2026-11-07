// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/log/backend/serial_budget.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/log/backend/serial_budget.rs"]
pub mod serial_budget;

pub fn budget_new() -> serial_budget::Budget {
    serial_budget::Budget::new()
}

