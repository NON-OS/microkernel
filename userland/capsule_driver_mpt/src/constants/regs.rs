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

//! The MPI 1.x system interface (LSI's MPI specification; Linux's
//! mpi.h, mpi_ioc.h; QEMU's hw/scsi/mptsas.c, VirtualBox's
//! DevLsiLogicSCSI.cpp).

pub const REG_DOORBELL: usize = 0x00;
pub const REG_HOST_INTERRUPT_STATUS: usize = 0x30;
pub const REG_HOST_INTERRUPT_MASK: usize = 0x34;
/// Request frame addresses are posted here; on read, the reply post FIFO
/// gives a completion, and on write the reply free FIFO takes a frame back.
pub const REG_REQUEST_FIFO: usize = 0x40;
pub const REG_REPLY_FIFO: usize = 0x44;

/// The doorbell: the IOC's state in the top nibble (0 reset, 1 ready, 2
/// operational, 4 fault), and "in use".
pub const IOC_STATE_MASK: u32 = 0xF000_0000;
pub const IOC_STATE_READY: u32 = 0x1000_0000;
pub const IOC_STATE_OPERATIONAL: u32 = 0x2000_0000;
pub const DOORBELL_ACTIVE: u32 = 0x0800_0000;
pub const DOORBELL_FUNCTION_SHIFT: u32 = 24;
pub const DOORBELL_DWORDS_SHIFT: u32 = 16;

/// The host interrupt status: the IOC has not taken the doorbell yet, a
/// reply is posted, the doorbell has something for the host.
pub const HIS_IOP_DOORBELL_STATUS: u32 = 0x8000_0000;
pub const HIS_DOORBELL_INTERRUPT: u32 = 0x0000_0001;
/// Every interrupt masked: the doorbell's and the reply's.
pub const HIM_ALL: u32 = 0x0000_0009;

/// The reply post FIFO reads this when empty.
pub const FIFO_EMPTY: u32 = 0xFFFF_FFFF;
/// An address reply: the frame's address, shifted right by one.
pub const ADDRESS_REPLY: u32 = 0x8000_0000;

pub const FUNCTION_SCSI_IO_REQUEST: u8 = 0x00;
pub const FUNCTION_IOC_INIT: u8 = 0x02;
pub const FUNCTION_IOC_FACTS: u8 = 0x03;
pub const FUNCTION_PORT_ENABLE: u8 = 0x06;
pub const FUNCTION_IOC_MESSAGE_UNIT_RESET: u8 = 0x40;
pub const FUNCTION_HANDSHAKE: u8 = 0x42;

/// IOCStatus, with the log info flag masked off.
pub const IOCSTATUS_MASK: u16 = 0x7FFF;
pub const IOCSTATUS_SUCCESS: u16 = 0x0000;
pub const IOCSTATUS_BUSY: u16 = 0x0002;
pub const IOCSTATUS_SCSI_RECOVERED_ERROR: u16 = 0x0040;
pub const IOCSTATUS_SCSI_INVALID_BUS: u16 = 0x0041;
pub const IOCSTATUS_SCSI_INVALID_TARGETID: u16 = 0x0042;
pub const IOCSTATUS_SCSI_DEVICE_NOT_THERE: u16 = 0x0043;
pub const IOCSTATUS_SCSI_DATA_UNDERRUN: u16 = 0x0045;
pub const IOCSTATUS_SCSI_TASK_TERMINATED: u16 = 0x0048;
pub const IOCSTATUS_SCSI_IOC_TERMINATED: u16 = 0x004B;
pub const IOCSTATUS_SCSI_EXT_TERMINATED: u16 = 0x004C;

/// SCSIState: the sense in the sense buffer is the command's.
pub const SCSI_STATE_AUTOSENSE_VALID: u8 = 0x01;
