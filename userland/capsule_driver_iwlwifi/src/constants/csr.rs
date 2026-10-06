// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! The CSR registers setup reads and writes, and their bits.

pub const CSR_INT_COALESCING: usize = 0x004;
pub const CSR_INT: usize = 0x008;
pub const CSR_INT_MASK: usize = 0x00C;
pub const CSR_FH_INT_STATUS: usize = 0x010;
pub const CSR_GP_CNTRL: usize = 0x024;
pub const CSR_HW_REV: usize = 0x028;
// The CSR_GP_CNTRL layout of Linux iwl_csr_v1, which covers every family
// probed here up to AX210. Bit 1 is undefined there: polling it for the MAC
// clock timed out on every card, so setup never got past this register.
// Bz-family parts (BE200, 0x272B) use the v2 layout, so probe refuses them.
pub const GP_CNTRL_MAC_CLOCK_READY: u32 = 0x0000_0001;
pub const GP_CNTRL_INIT_DONE: u32 = 0x0000_0004;
pub const GP_CNTRL_MAC_ACCESS_REQ: u32 = 0x0000_0008;
pub const GP_CNTRL_XTAL_ON: u32 = 0x0000_0400;
/// Set while the hardware RF-kill switch lets the radio on.
pub const GP_CNTRL_HW_RF_KILL_SW: u32 = 0x0800_0000;
pub const ALL_INTS_MASK: u32 = 0xFFFF_FFFF;
pub const INT_MASK_DISABLED: u32 = 0;
pub const INT_COALESCING_TIMEOUT: u32 = 64;
/// How long setup waits for the MAC clock. Linux `iwl_finish_nic_init` gives
/// 25 ms once it has prepared the card; setup asks before that preparation,
/// so it allows ten times as long.
pub const CLOCK_READY_MS: u64 = 250;
pub const INT_BIT_ALIVE: u32 = 1 << 0;
pub const ALIVE_POLL_ITERS: usize = 2_000_000;
