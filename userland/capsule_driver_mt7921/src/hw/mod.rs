/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The register layer: BAR0, the address map onto it, and bounded polls.

pub mod addr;
pub mod bus;
pub mod map;
pub mod regs;

pub use bus::Mmio;
pub use regs::Regs;

/// `mt76_poll_msec`: whether `addr & mask` reads `want` within `ms`.
pub fn poll(regs: &Regs, addr: u32, mask: u32, want: u32, ms: u64) -> bool {
    crate::clock::until_ms(ms, || regs.rr(addr) & mask == want)
}
