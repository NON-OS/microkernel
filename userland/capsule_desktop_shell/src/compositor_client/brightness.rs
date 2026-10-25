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

//! NCMP 0x0014 BRIGHTNESS_STEP: a brightness key, the focused output's
//! panel moved a level; the answer's status and the level it is at.

use super::wire::call_payload_boot;

const OP: u16 = 0x0014;

/// Step `up` or down; None when the compositor did not answer.
pub fn step(compositor_port: u32, request_id: u32, up: bool) -> Option<(i32, u32)> {
    let mut level = [0u8; 4];
    let payload = u32::from(up).to_le_bytes();
    let status = call_payload_boot(compositor_port, OP, request_id, &payload, &mut level).ok()?;
    Some((status, u32::from_le_bytes(level)))
}
