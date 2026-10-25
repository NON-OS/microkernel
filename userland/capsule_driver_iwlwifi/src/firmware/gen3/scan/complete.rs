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

//! A scan's completion, read back by uid.

/// A scan completion: its uid and status (`IWL_SCAN_OFFLOAD_COMPLETED` is 1,
/// `_ABORTED` 2). `None` for a payload shorter than the structure.
pub fn complete(payload: &[u8]) -> Option<(u32, u8)> {
    if payload.len() < 16 {
        return None;
    }
    let uid = u32::from_le_bytes([payload[0], payload[1], payload[2], payload[3]]);
    Some((uid, payload[6]))
}
