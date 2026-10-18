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

//! The MultiMaster host adapter interface (the BT-958 technical reference,
//! Linux's BusLogic.h, VirtualBox's DevBusLogic.cpp).

/// Control on write, status on read.
pub const REG_CONTROL: u16 = 0;
pub const REG_STATUS: u16 = 0;
/// Command and parameter bytes on write, reply bytes on read.
pub const REG_COMMAND: u16 = 1;
pub const REG_DATA_IN: u16 = 1;
pub const REG_INTERRUPT: u16 = 2;

pub const CTL_HARD_RESET: u8 = 0x80;
pub const CTL_INTERRUPT_RESET: u8 = 0x20;

/// Diagnostic active, diagnostic failure, initialization (mailboxes)
/// required, host adapter ready, command/parameter register busy, data in
/// register ready, command invalid.
pub const ST_DACT: u8 = 0x80;
pub const ST_DFAIL: u8 = 0x40;
pub const ST_INREQ: u8 = 0x20;
pub const ST_HARDY: u8 = 0x10;
pub const ST_CPRBSY: u8 = 0x08;
pub const ST_DIRRDY: u8 = 0x04;
pub const ST_CMDINV: u8 = 0x01;

/// The command complete interrupt bit, latched whether the line is enabled
/// or not.
pub const INT_CMDC: u8 = 0x04;

pub const CMD_START_MAILBOX: u8 = 0x02;
pub const CMD_INQUIRE_CONFIGURATION: u8 = 0x0B;
pub const CMD_DISABLE_INTERRUPT: u8 = 0x25;
pub const CMD_INIT_EXTENDED_MAILBOX: u8 = 0x81;

/// A 32-bit mailbox entry: the CCB's address, then (outgoing) the action
/// code in the last byte, or (incoming) the adapter status, the target
/// status and the completion code.
pub const MBOX_BYTES: usize = 8;
pub const MBO_ACTION: usize = 7;
pub const ACTION_START: u8 = 0x01;
pub const MBI_HOST_STATUS: usize = 4;
pub const MBI_TARGET_STATUS: usize = 5;
pub const MBI_COMPLETION: usize = 7;
pub const COMP_FREE: u8 = 0x00;
pub const COMP_DONE: u8 = 0x01;
pub const COMP_ABORTED: u8 = 0x02;
pub const COMP_ABORTED_NOT_FOUND: u8 = 0x03;
pub const COMP_ERROR: u8 = 0x04;

/// The 32-bit command control block's fields.
pub const CCB_BYTES: usize = 40;
pub const CCB_OPCODE: usize = 0;
pub const CCB_DIRECTION: usize = 1;
pub const CCB_CDB_LEN: usize = 2;
pub const CCB_SENSE_LEN: usize = 3;
pub const CCB_DATA_LEN: usize = 4;
pub const CCB_DATA: usize = 8;
pub const CCB_TARGET: usize = 16;
pub const CCB_LUN: usize = 17;
pub const CCB_CDB: usize = 18;
pub const CCB_SENSE: usize = 36;
/// An initiator CCB that reports the residual in its data length.
pub const OP_INITIATOR_RESIDUAL: u8 = 0x03;
/// Bits 3 and 4 of the direction byte: checked in, checked out, no data.
pub const DIR_IN: u8 = 1 << 3;
pub const DIR_OUT: u8 = 2 << 3;
pub const DIR_NONE: u8 = 3 << 3;
/// A CCB carries the CDB in place, at most twelve bytes.
pub const CDB_MAX: usize = 12;

/// The host adapter statuses the driver tells apart.
pub const BT_SUCCESS: u8 = 0x00;
pub const BT_LINKED_COMPLETE: u8 = 0x0A;
pub const BT_LINKED_COMPLETE_FLAG: u8 = 0x0B;
pub const BT_SELECTION_TIMEOUT: u8 = 0x11;
/// Data overrun or underrun: an underrun, with the residual given.
pub const BT_DATA_RUN: u8 = 0x12;
pub const BT_BUS_FREE: u8 = 0x13;
pub const BT_HA_RESET: u8 = 0x22;
pub const BT_OTHER_RESET: u8 = 0x23;

/// A wide bus's targets; LUNs to 7.
pub const MAX_TARGET: u16 = 15;
pub const MAX_LUN: u16 = 7;
