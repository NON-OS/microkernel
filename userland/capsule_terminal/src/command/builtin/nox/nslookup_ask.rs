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

//! The request that asks the DNS service for a host's address.

use alloc::vec::Vec;

use super::HDR_LEN;

const MAGIC: u32 = 0x4E44_4E53;
const OP_RESOLVE_A: u16 = 2;

/// One A lookup of `host`, header first.
pub(super) fn request(host: &[u8]) -> Vec<u8> {
    let mut tx: Vec<u8> = Vec::with_capacity(HDR_LEN + host.len());
    tx.extend_from_slice(&MAGIC.to_le_bytes());
    tx.extend_from_slice(&1u16.to_le_bytes());
    tx.extend_from_slice(&OP_RESOLVE_A.to_le_bytes());
    tx.extend_from_slice(&0u32.to_le_bytes());
    tx.extend_from_slice(&1u32.to_le_bytes());
    tx.extend_from_slice(&(host.len() as u32).to_le_bytes());
    tx.extend_from_slice(host);
    tx
}
