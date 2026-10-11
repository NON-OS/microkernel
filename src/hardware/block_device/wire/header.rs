// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

//! The block wire every new block driver speaks (SD/eMMC, UFS, card
//! readers): the 20-byte capsule header under its own magic, with the unit
//! (which disk behind the endpoint) where the shared header keeps a
//! reserved word, five ops, and a reply that repeats the header, then a
//! status, then the payload.

/// "NBW1".
pub(super) const MAGIC: u32 = 0x4E42_5731;
pub(super) const VERSION: u16 = 1;
pub(super) const HDR_LEN: usize = 20;

pub(super) const OP_HEALTHCHECK: u16 = 1;
pub(super) const OP_GEOMETRY: u16 = 2;
pub(super) const OP_READ: u16 = 3;
pub(super) const OP_WRITE: u16 = 4;
pub(super) const OP_FLUSH: u16 = 5;
pub(super) const OP_IDENTIFY: u16 = 6;

/// lba u64, count u32: the head of a READ or WRITE request.
pub(super) const RW_HEADER_LEN: usize = 12;
/// lba_count u64, lba_bytes u32, max_transfer_bytes u32, flags u32.
pub(super) const GEOMETRY_LEN: usize = 20;
/// model_len, serial_len, medium, reserved, model[40], serial[20].
pub(super) const IDENTIFY_LEN: usize = 64;

/// The most one READ or WRITE moves, whatever the driver offers: a reply
/// this size is still a small part of an IPC message.
pub(super) const MAX_TRANSFER_BYTES: u32 = 128 * 1024;
