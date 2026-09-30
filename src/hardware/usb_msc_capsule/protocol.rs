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

//! The wire the kernel client and driver.usb_msc0 speak, mirrored at
//! `userland/capsule_driver_usb_msc/src/protocol/{header,ops,limits,errno}.rs`.

use alloc::vec::Vec;

use crate::services::lifecycle::transport::{self, DecodedResponse};

const MAGIC: u32 = 0x4E55_4D53;
const VERSION: u16 = 1;
pub(super) const OP_BLK_CAPACITY: u16 = 0x0020;
pub(super) const OP_BLK_READ: u16 = 0x0021;
pub(super) const OP_BLK_WRITE: u16 = 0x0022;
pub(super) const OP_BLK_FLUSH: u16 = 0x0023;
pub(super) const SECTOR_SIZE: usize = 512;
/// Sectors one request moves: 64, 32 KiB, as the driver caps them.
pub(super) const MAX_RW_BYTES: usize = 64 * SECTOR_SIZE;
const MAX_PAYLOAD_BYTES: u32 = MAX_RW_BYTES as u32 + 64;

pub(super) const E_NXIO: i32 = -6;
pub(super) const E_AGAIN: i32 = -11;
pub(super) const E_ACCES: i32 = -13;
pub(super) const E_NODEV: i32 = -19;
pub(super) const E_INVAL: i32 = -22;
pub(super) const E_MSGSIZE: i32 = -90;
pub(super) const E_NOTSUP: i32 = -95;

pub(super) fn encode_request(op: u16, request_id: u32, body: &[u8]) -> Vec<u8> {
    transport::encode_request(MAGIC, VERSION, op, 0, request_id, body)
}

pub(super) fn decode_response(buf: &[u8]) -> Option<DecodedResponse<'_>> {
    transport::decode_v1_response(buf, MAGIC, VERSION, MAX_PAYLOAD_BYTES)
}
