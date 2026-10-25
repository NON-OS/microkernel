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

//! The message frames the driver sends and the replies it reads (MPI 1.5,
//! mpi_ioc.h and mpi_init.h). Offsets in bytes, fields little endian.

/// Every message: function in byte 3, the context in bytes 8..12.
pub const MSG_FUNCTION: usize = 3;
pub const MSG_CONTEXT: usize = 8;
/// Every reply: its length in dwords in byte 2 (the handshake reads it as
/// it comes), the IOC status in 14..16.
pub const REPLY_IOC_STATUS: usize = 14;

/// IOC FACTS: the header alone, three dwords.
pub const FACTS_DWORDS: usize = 3;
/// IOC FACTS reply fields the driver reads.
pub const FACTS_REQUEST_FRAME_SIZE: usize = 0x1A;
pub const FACTS_MAX_DEVICES: usize = 0x2E;
pub const FACTS_MAX_BUSES: usize = 0x2F;
/// The longest reply the handshake keeps: the facts reply, twenty dwords.
pub const HANDSHAKE_REPLY_MAX: usize = 0x50;

/// IOC INIT: who, max devices and buses, the reply frame size, the high
/// halves of the frame and sense addresses, the MPI version. Eleven dwords.
pub const INIT_DWORDS: usize = 11;
pub const INIT_WHO: usize = 0;
pub const INIT_MAX_DEVICES: usize = 5;
pub const INIT_MAX_BUSES: usize = 6;
pub const INIT_REPLY_FRAME_SIZE: usize = 12;
pub const INIT_MSG_VERSION: usize = 40;
pub const INIT_HEADER_VERSION: usize = 42;
pub const WHOINIT_HOST_DRIVER: u8 = 0x04;
pub const MPI_VERSION: u16 = 0x0105;
pub const MPI_HEADER_VERSION: u16 = 0x0E00;

/// PORT ENABLE, three dwords: port 0.
pub const PORT_ENABLE_DWORDS: usize = 3;

/// SCSI IO REQUEST.
pub const IO_TARGET: usize = 0;
pub const IO_BUS: usize = 1;
pub const IO_CDB_LEN: usize = 4;
pub const IO_SENSE_LEN: usize = 5;
pub const IO_LUN: usize = 12;
pub const IO_CONTROL: usize = 20;
pub const IO_CDB: usize = 24;
pub const IO_DATA_LEN: usize = 40;
pub const IO_SENSE_ADDR: usize = 44;
pub const IO_SGL: usize = 48;
/// The chain offset field, in dwords: zero, the one SGE is in the frame.
pub const IO_CDB_MAX: usize = 16;
pub const CONTROL_NO_DATA: u32 = 0x0000_0000;
pub const CONTROL_WRITE: u32 = 0x0100_0000;
pub const CONTROL_READ: u32 = 0x0200_0000;
/// One simple SGE, 32-bit address, the last of its buffer and of the
/// list; host to IOC for a write.
pub const SGE_SIMPLE_LAST: u32 = (0x80 | 0x40 | 0x10 | 0x01) << 24;
pub const SGE_HOST_TO_IOC: u32 = 0x04 << 24;

/// SCSI IO reply fields.
pub const IOR_SCSI_STATUS: usize = 12;
pub const IOR_SCSI_STATE: usize = 13;
pub const IOR_TRANSFER_COUNT: usize = 20;
pub const IOR_SENSE_COUNT: usize = 24;
