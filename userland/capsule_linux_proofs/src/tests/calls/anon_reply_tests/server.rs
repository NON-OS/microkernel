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

//! A reply laid out as net.anon's front lays one.

use crate::linux::net::anon_ops::*;

/// A reply as net.anon's respond.rs writes it.
pub(super) fn server_reply(op: u16, status: u16, request_id: u32, body: &[u8]) -> Vec<u8> {
    let mut tx = Vec::new();
    tx.extend_from_slice(&MAGIC.to_le_bytes());
    tx.extend_from_slice(&VERSION.to_le_bytes());
    tx.extend_from_slice(&op.to_le_bytes());
    tx.extend_from_slice(&status.to_le_bytes());
    tx.extend_from_slice(&[0, 0]);
    tx.extend_from_slice(&request_id.to_le_bytes());
    tx.extend_from_slice(&(body.len() as u32).to_le_bytes());
    tx.extend_from_slice(body);
    tx
}
