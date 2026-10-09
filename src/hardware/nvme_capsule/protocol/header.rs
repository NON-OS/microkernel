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

pub(in super::super) const MAGIC: u32 = 0x4E4E_564D;
pub(in super::super) const VERSION: u16 = 1;

pub(in super::super) const CONTROLLER_INFO_PAYLOAD_LEN: usize = 52;
pub(in super::super) const IDENTIFY_CONTROLLER_PAYLOAD_LEN: usize = 88;
pub(in super::super) const IDENTIFY_NAMESPACE_PAYLOAD_LEN: usize = 36;
pub(in super::super) const SMART_HEALTH_PAYLOAD_LEN: usize = 177;
// The capsule's data buffer, the most one read or write moves: 64 LBAs of
// 512 bytes or 8 of 4096. The wire counts the namespace's own LBAs, never
// 512-byte sectors; client::lba_map maps the block layer's sectors onto them.
const DATA_BUFFER_SECTORS: u32 = 64;
const DATA_BUFFER_SECTOR_BYTES: u32 = 512;
pub(in super::super) const MAX_RW_PAYLOAD_BYTES: u32 =
    DATA_BUFFER_SECTORS * DATA_BUFFER_SECTOR_BYTES;
const _: () = assert!(MAX_RW_PAYLOAD_BYTES as usize > SMART_HEALTH_PAYLOAD_LEN);
pub(in super::super) const MAX_PAYLOAD_BYTES: u32 = MAX_RW_PAYLOAD_BYTES;
