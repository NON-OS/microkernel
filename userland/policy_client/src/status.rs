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

//! The kernel hardening record the store forwards.

use nonos_policy_proto::{IPC_PAYLOAD_MAX, KIND_BYTES, OP_STATUS};

use super::call::call;

/// Copy the hardening record into `out`, returning whether it arrived whole.
///
pub fn status(port: u32, out: &mut [u8]) -> bool {
    let mut rx = [0u8; IPC_PAYLOAD_MAX];
    let reply = match call(port, OP_STATUS, 0, KIND_BYTES, &mut rx) {
        Some(r) => r,
        None => return false,
    };
    if reply.header.kind != KIND_BYTES || reply.payload.len() != out.len() {
        return false;
    }
    out.copy_from_slice(reply.payload);
    true
}
