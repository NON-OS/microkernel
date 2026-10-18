// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! The setup wait for the MAC clock is held to milliseconds, not to a count
//! of register reads.

use std::time::{Duration, Instant};

use crate::constants::{CLOCK_READY_MS, CSR_GP_CNTRL, GP_CNTRL_MAC_CLOCK_READY};
use crate::regs::Regs;

#[test]
fn a_clock_that_never_comes_is_waited_for_its_milliseconds_then_refused() {
    let mut window = vec![0u32; 64];
    // SAFETY: the window's own 256 bytes, alive for the test.
    let regs = unsafe { Regs::new(window.as_mut_ptr() as u64, 256) };
    let start = Instant::now();
    assert!(!regs.poll_set(CSR_GP_CNTRL, GP_CNTRL_MAC_CLOCK_READY, 30));
    let took = start.elapsed();
    assert!(took >= Duration::from_millis(30), "waited the whole time: {took:?}");
    assert!(took < Duration::from_millis(2000), "and not much more: {took:?}");
}

#[test]
fn a_ready_clock_is_taken_at_once() {
    let mut window = vec![0u32; 64];
    // SAFETY: the window's own 256 bytes, alive for the test.
    let regs = unsafe { Regs::new(window.as_mut_ptr() as u64, 256) };
    regs.write32(CSR_GP_CNTRL, GP_CNTRL_MAC_CLOCK_READY);
    let start = Instant::now();
    assert!(regs.poll_set(CSR_GP_CNTRL, GP_CNTRL_MAC_CLOCK_READY, CLOCK_READY_MS));
    assert!(start.elapsed() < Duration::from_millis(CLOCK_READY_MS));
}

#[test]
fn a_register_past_the_window_or_unaligned_is_never_touched() {
    let mut window = vec![0x1234_5678u32; 65];
    // SAFETY: the first 256 of the window's 260 bytes, alive for the test.
    let regs = unsafe { Regs::new(window.as_mut_ptr() as u64, 256) };
    assert_eq!(regs.read32(252), 0x1234_5678, "the last register");
    for off in [256, 257, usize::MAX - 3, usize::MAX, 2] {
        assert_eq!(regs.read32(off), u32::MAX, "{off}: reads as nothing there");
        regs.write32(off, 0);
    }
    assert!(window.iter().all(|&w| w == 0x1234_5678), "no write went anywhere");
}
