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

//! What the tests read back: TLVs with a firmware id, and the opcodes
//! sent after a given one.

use super::model::Intel;

/// The TLVs with a firmware id after them, when `id` is not empty.
pub fn named(mut v: Vec<u8>, id: &[u8]) -> Vec<u8> {
    if !id.is_empty() {
        v.extend([&[0x50, id.len() as u8][..], id].concat());
    }
    v
}

pub fn ops_after(m: &Intel, op: u16) -> Vec<u16> {
    let at = m.ops.iter().rposition(|&o| o == op).expect("sent");
    m.ops[at + 1..].to_vec()
}
