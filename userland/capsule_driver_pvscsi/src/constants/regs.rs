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

//! The PVSCSI register window and its commands (Linux's vmw_pvscsi.h, which
//! QEMU's hw/scsi/vmw_pvscsi.c follows).

pub const REG_COMMAND: usize = 0x0;
pub const REG_COMMAND_DATA: usize = 0x4;
pub const REG_COMMAND_STATUS: usize = 0x8;
pub const REG_INTR_MASK: usize = 0x2010;
/// A doorbell for every request but a read or a write, and one for those.
pub const REG_KICK_NON_RW_IO: usize = 0x3014;
pub const REG_KICK_RW_IO: usize = 0x4018;

pub const CMD_ADAPTER_RESET: u32 = 1;
pub const CMD_SETUP_RINGS: u32 = 3;
/// A command the adapter carried out.
pub const COMMAND_OK: u32 = 0;

/// The rings state page: the request ring's producer, consumer (4, the
/// adapter's) and size as a power of two, then the completion ring's.
pub const RS_REQ_PROD: usize = 0;
pub const RS_REQ_LOG2: usize = 8;
pub const RS_CMP_PROD: usize = 12;
pub const RS_CMP_CONS: usize = 16;
pub const RS_CMP_LOG2: usize = 20;

/// struct PVSCSIRingReqDesc, 128 bytes.
pub const REQ_BYTES: usize = 128;
pub const REQ_CONTEXT: usize = 0;
pub const REQ_DATA_ADDR: usize = 8;
pub const REQ_DATA_LEN: usize = 16;
pub const REQ_SENSE_ADDR: usize = 24;
pub const REQ_SENSE_LEN: usize = 32;
pub const REQ_FLAGS: usize = 36;
pub const REQ_CDB: usize = 40;
pub const REQ_CDB_LEN: usize = 56;
pub const REQ_LUN: usize = 57;
pub const REQ_BUS: usize = 66;
pub const REQ_TARGET: usize = 67;
/// The largest CDB a request carries in place.
pub const CDB_MAX: usize = 16;

pub const FLAG_DIR_NONE: u32 = 1 << 2;
pub const FLAG_DIR_TOHOST: u32 = 1 << 3;
pub const FLAG_DIR_TODEVICE: u32 = 1 << 4;

/// struct PVSCSIRingCmpDesc, 32 bytes.
pub const CMP_BYTES: usize = 32;
pub const CMP_CONTEXT: usize = 0;
pub const CMP_DATA_LEN: usize = 8;
pub const CMP_SENSE_LEN: usize = 16;
pub const CMP_HOST_STATUS: usize = 20;
pub const CMP_SCSI_STATUS: usize = 22;

/// The host adapter statuses (BTSTAT_*) the driver tells apart.
pub const BT_SUCCESS: u16 = 0x00;
pub const BT_LINKED_COMMAND_COMPLETED: u16 = 0x0A;
pub const BT_LINKED_COMMAND_COMPLETED_WITH_FLAG: u16 = 0x0B;
pub const BT_DATA_UNDERRUN: u16 = 0x0C;
/// Selection timed out: nothing at that target.
pub const BT_SELTIMEO: u16 = 0x11;
pub const BT_BUSFREE: u16 = 0x13;
pub const BT_SENTRST: u16 = 0x22;
pub const BT_RECVRST: u16 = 0x23;
pub const BT_BUSRESET: u16 = 0x25;
pub const BT_ABORTQUEUE: u16 = 0x26;
pub const BT_HATIMEOUT: u16 = 0x30;

/// Targets a PVSCSI bus has; the LUN goes in one byte.
pub const MAX_TARGET: u16 = 63;
pub const MAX_LUN: u16 = 255;
