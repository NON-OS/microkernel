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

//! virtio-scsi (virtio 1.2, 5.6): the device configuration, the queues, the
//! command request and its response.

// Device configuration offsets.
pub const CFG_NUM_QUEUES: usize = 0;
pub const CFG_MAX_SECTORS: usize = 8;
pub const CFG_SENSE_SIZE: usize = 20;
pub const CFG_CDB_SIZE: usize = 24;
pub const CFG_MAX_TARGET: usize = 30;
pub const CFG_MAX_LUN: usize = 32;

/// Queue 0 is control, 1 events, 2 and up requests; the driver uses 2.
pub const REQUESTQ: u16 = 2;

/// The sizes the driver sets, which are the specification's defaults.
pub const CDB_SIZE: usize = 32;
pub const SENSE_SIZE: usize = 96;

/// struct virtio_scsi_req_cmd: lun[8], id u64, task_attr, prio, crn, cdb.
pub const REQ_LUN: usize = 0;
pub const REQ_ID: usize = 8;
pub const REQ_CDB: usize = 19;
pub const REQ_LEN: usize = REQ_CDB + CDB_SIZE;
/// struct virtio_scsi_resp_cmd: sense_len u32, resid u32,
/// status_qualifier u16, status, response, sense.
pub const RESP_SENSE_LEN: usize = 0;
pub const RESP_RESID: usize = 4;
pub const RESP_STATUS: usize = 10;
pub const RESP_RESPONSE: usize = 11;
pub const RESP_SENSE: usize = 12;
pub const RESP_LEN: usize = RESP_SENSE + SENSE_SIZE;

/// The response codes the driver tells apart; every other one (an overrun,
/// a target or nexus failure, a plain failure) fails the command.
pub const S_OK: u8 = 0;
pub const S_ABORTED: u8 = 2;
pub const S_BAD_TARGET: u8 = 3;
pub const S_RESET: u8 = 4;
pub const S_BUSY: u8 = 5;
pub const S_TRANSPORT_FAILURE: u8 = 6;

/// The LUN field: 1, the target, then the LUN in flat space addressing.
pub const fn lun_field(target: u16, lun: u16) -> [u8; 8] {
    [1, target as u8, 0x40 | (lun >> 8) as u8 & 0x3F, lun as u8, 0, 0, 0, 0]
}

/// The targets a virtio-scsi LUN field can name.
pub const MAX_TARGET: u16 = 255;
/// The LUNs flat space addressing names.
pub const MAX_LUN: u16 = 0x3FFF;
