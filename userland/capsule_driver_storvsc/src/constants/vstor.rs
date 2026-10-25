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

//! The storage protocol over the channel (Linux's storvsc_drv.c: struct
//! vstor_packet, struct vmscsi_request), every packet 64 bytes:
//!
//! ```text
//!   0 operation u32   4 flags u32   8 status u32   12 the body (52 bytes)
//!   vmscsi_request at 12:
//!     12 length u16  14 srb_status  15 scsi_status  16 port  17 path_id
//!     18 target_id   19 lun         20 cdb_length   21 sense_info_length
//!     22 data_in     24 data_transfer_length u32    28 cdb or sense (20)
//!     48 reserve u16 50 queue_tag   51 queue_action 52 srb_flags u32
//!     56 time_out_value u32         60 queue_sort u32
//!   vmstorage_protocol_version at 12: major_minor u16, revision u16
//!   vmstorage_channel_properties at 12: max_transfer_bytes at 24
//! ```

pub const PACKET: usize = 64;
/// sizeof(struct vmscsi_request) from WIN8 on.
pub const SRB_LENGTH: u16 = 48;

pub const OP_COMPLETE_IO: u32 = 1;
pub const OP_REMOVE_DEVICE: u32 = 2;
pub const OP_EXECUTE_SRB: u32 = 3;
pub const OP_RESET_BUS: u32 = 6;
pub const OP_BEGIN_INIT: u32 = 7;
pub const OP_END_INIT: u32 = 8;
pub const OP_QUERY_PROTOCOL_VERSION: u32 = 9;
pub const OP_QUERY_PROPERTIES: u32 = 10;
pub const OP_ENUMERATE_BUS: u32 = 11;

/// REQUEST_COMPLETION_FLAG: the host answers with a completion.
pub const FLAG_REQUEST_COMPLETION: u32 = 1;

/// WIN10 (6.2) then WIN8.1 (6.0), Linux's protocol_version list: hosts
/// older than these speak a shorter request.
pub const VERSIONS: [u16; 2] = [0x0602, 0x0600];

pub const DATA_WRITE: u8 = 0;
pub const DATA_READ: u8 = 1;
pub const DATA_UNKNOWN: u8 = 2;

pub const CDB_MAX: usize = 16;
pub const SENSE_BYTES: usize = 20;

pub const SRB_FLAGS_QUEUE_ACTION_ENABLE: u32 = 0x2;
pub const SRB_FLAGS_DISABLE_SYNCH_TRANSFER: u32 = 0x8;
pub const SRB_FLAGS_DATA_IN: u32 = 0x40;
pub const SRB_FLAGS_DATA_OUT: u32 = 0x80;
pub const SRB_FLAGS_NO_QUEUE_FREEZE: u32 = 0x100;
/// SP_UNTAGGED and SRB_SIMPLE_TAG_REQUEST.
pub const QUEUE_TAG_UNTAGGED: u8 = 0xFF;
pub const QUEUE_ACTION_SIMPLE: u8 = 0x20;
pub const TIME_OUT_S: u32 = 60;

/// The SRB statuses read by name; any other (ERROR 0x04, INVALID_REQUEST
/// 0x06, INTERNAL_ERROR 0x30, ...) is a failure.
pub const SRB_STATUS_SUCCESS: u8 = 0x01;
pub const SRB_STATUS_ABORTED: u8 = 0x02;
pub const SRB_STATUS_TIMEOUT: u8 = 0x09;
pub const SRB_STATUS_SELECTION_TIMEOUT: u8 = 0x0A;
pub const SRB_STATUS_BUS_RESET: u8 = 0x0E;
pub const SRB_STATUS_DATA_OVERRUN: u8 = 0x12;
pub const SRB_STATUS_INVALID_LUN: u8 = 0x20;
pub const SRB_STATUS_QUEUE_FROZEN: u8 = 0x40;
pub const SRB_STATUS_AUTOSENSE_VALID: u8 = 0x80;

/// The host's own requests and resets travel on ids no SCSI request takes:
/// those count up from 1, and 0 is the host's unsolicited packet.
pub const INIT_ID: u64 = u64::MAX - 1;
pub const RESET_ID: u64 = u64::MAX - 2;
