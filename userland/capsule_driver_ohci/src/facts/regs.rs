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

//! The operational registers, byte offsets into the memory BAR, and the
//! bits this driver uses.

pub const HC_REVISION: u32 = 0x00;
pub const HC_CONTROL: u32 = 0x04;
pub const HC_COMMAND_STATUS: u32 = 0x08;
pub const HC_INTERRUPT_STATUS: u32 = 0x0C;
pub const HC_INTERRUPT_DISABLE: u32 = 0x14;
pub const HC_HCCA: u32 = 0x18;
pub const HC_CONTROL_HEAD_ED: u32 = 0x20;
pub const HC_CONTROL_CURRENT_ED: u32 = 0x24;
pub const HC_BULK_HEAD_ED: u32 = 0x28;
pub const HC_BULK_CURRENT_ED: u32 = 0x2C;
pub const HC_FM_INTERVAL: u32 = 0x34;
pub const HC_FM_NUMBER: u32 = 0x3C;
pub const HC_PERIODIC_START: u32 = 0x40;
pub const HC_LS_THRESHOLD: u32 = 0x44;
/// The legacy keyboard emulation's control, present when HcRevision's
/// legacy bit is set.
pub const HCE_CONTROL: u32 = 0x100;

/// HcRevision: legacy support present.
pub const REV_LEGACY: u32 = 1 << 8;

/// HcControl: control/bulk service ratio 4:1, periodic, control and bulk
/// lists enabled, the functional state, and the SMM owner flag.
pub const CTL_CBSR_4: u32 = 3;
pub const CTL_PLE: u32 = 1 << 2;
pub const CTL_CLE: u32 = 1 << 4;
pub const CTL_BLE: u32 = 1 << 5;
pub const CTL_HCFS: u32 = 3 << 6;
pub const HCFS_OPERATIONAL: u32 = 2 << 6;
pub const CTL_IR: u32 = 1 << 8;

/// HcCommandStatus: host reset, control and bulk lists filled, and the
/// ownership change request.
pub const CS_HCR: u32 = 1 << 0;
pub const CS_CLF: u32 = 1 << 1;
pub const CS_BLF: u32 = 1 << 2;
pub const CS_OCR: u32 = 1 << 3;

/// HcInterruptStatus/Disable: writeback done head, and every source with
/// the master enable.
pub const INT_WDH: u32 = 1 << 1;
pub const INT_ALL: u32 = 0xC000_007F;

/// HcFmInterval: the frame interval (bit times less one), the largest
/// data packet, and the toggle that marks a new value.
pub const FI_MASK: u32 = 0x3FFF;
pub const FI_DEFAULT: u32 = 11_999;
pub const FSMPS_SHIFT: u32 = 16;
pub const FIT: u32 = 1 << 31;
/// HcLSThreshold's reset value: the last bit time a low-speed packet of
/// eight bytes may start in.
pub const LS_THRESHOLD: u32 = 0x628;
