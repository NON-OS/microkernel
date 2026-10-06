// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! The legacy FH service-channel load registers.

// Flow Handler (FH) registers for the legacy (pre-8000) firmware-load DMA path.
// The FH service channel (9) transfers a staged section from host DRAM into the
// device's internal SRAM at a destination address. Offsets from iwl-fh.h.
pub const FH_TFDIB_CTRL0_REG: usize = 0x1948;
pub const FH_TFDIB_CTRL1_REG: usize = 0x194C;
pub const FH_SRVC_CHNL_SRAM_ADDR_REG: usize = 0x19C8;
pub const FH_TCSR_CHNL_TX_CONFIG_REG: usize = 0x1E20;
pub const FH_TCSR_CHNL_TX_BUF_STS_REG: usize = 0x1E28;
pub const FH_TFDIB_REG1_ADDR_BITSHIFT: u32 = 28;
pub const FH_TCSR_TX_CONFIG_DMA_PAUSE: u32 = 0x0000_0000;
pub const FH_TCSR_TX_CONFIG_DMA_ENABLE: u32 = 0x8000_0000;
pub const FH_TCSR_TX_CONFIG_CIRQ_HOST_ENDTFD: u32 = 0x0010_0000;
pub const FH_TCSR_TX_BUF_STS_TFDB_VALID: u32 = (1 << 20) | (1 << 12) | 0x1;
pub const FH_TX_POLL_ITERS: usize = 500_000;
