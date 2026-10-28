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
pub const ALL_INTS_MASK: u32 = 0xFFFF_FFFF;
pub const INT_MASK_DISABLED: u32 = 0;
pub const INT_COALESCING_TIMEOUT: u32 = 64;
/// How long setup waits for the MAC clock once the NIC is taken. Linux
/// `iwl_finish_nic_init` gives 25 ms; setup allows ten times as long.
pub const CLOCK_READY_MS: u64 = 250;
