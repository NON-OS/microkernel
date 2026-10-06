// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! The host registers in the memory BAR (rtsx_pci.h, RTSX_HCBAR to
//! RTSX_BIER), and the layout of the buffer the command and scatter-gather
//! engines read.

pub const HCBAR: u32 = 0x00;
pub const HCBCTLR: u32 = 0x04;
pub const HDBAR: u32 = 0x08;
pub const HDBCTLR: u32 = 0x0C;
pub const HAIMR: u32 = 0x10;
pub const BIPR: u32 = 0x14;
pub const BIER: u32 = 0x18;

/// BIPR, pending bits. Written back to clear, as rtsx_pci_isr does.
pub const TRANS_OK_INT: u32 = 1 << 29;
pub const TRANS_FAIL_INT: u32 = 1 << 28;
/// GPIO0_INT, which Linux also calls DELINK_INT: the link went down.
pub const DELINK_INT: u32 = 1 << 24;
pub const SD_EXIST: u32 = 1 << 16;

/// BIER, the same bit positions.
pub const TRANS_OK_INT_EN: u32 = 1 << 29;
pub const TRANS_FAIL_INT_EN: u32 = 1 << 28;
pub const MS_INT_EN: u32 = 1 << 26;
pub const SD_INT_EN: u32 = 1 << 25;
pub const SD_OC_INT_EN: u32 = 1 << 22;

/// HCBCTLR: start the command buffer, have the chip write read results
/// back into it, or stop it.
pub const START_CMD: u32 = 1 << 31;
pub const HW_AUTO_RSP: u32 = 1 << 30;
pub const STOP_CMD: u32 = 1 << 28;

/// HDBCTLR: start ADMA through the scatter-gather table, or stop it.
pub const TRIG_DMA: u32 = 1 << 31;
pub const ADMA_MODE: u32 = 0x02 << 26;
pub const STOP_DMA: u32 = 1 << 28;
pub const DEVICE_TO_HOST: u32 = 1;

/// HAIMR: one internal register access at a time.
pub const HAIMR_TRANS_START: u32 = 1 << 31;
pub const HAIMR_WRITE: u32 = 1 << 30;

/// The buffer Linux calls rtsx_resv_buf: commands first, the
/// scatter-gather table after them.
pub const RESV_BUF_LEN: u64 = 4096;
pub const HOST_CMDS_BUF_LEN: u64 = 1024;
