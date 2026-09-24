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

//! A text field.

use nonos_policy_proto::{Field, IPC_PAYLOAD_MAX, KIND_STR, OP_GET};

use crate::call::call;

/// Copy the stored text into `out` and return its length.
///
pub fn get_str(port: u32, field: Field, out: &mut [u8]) -> Option<usize> {
    let mut rx = [0u8; IPC_PAYLOAD_MAX];
    let reply = call(port, OP_GET, field as u32, KIND_STR, &mut rx)?;
    if reply.header.kind != KIND_STR || reply.header.field != field as u32 {
        return None;
    }
    let n = reply.payload.len();
    if n > out.len() {
        return None;
    }
    out[..n].copy_from_slice(reply.payload);
    Some(n)
}
