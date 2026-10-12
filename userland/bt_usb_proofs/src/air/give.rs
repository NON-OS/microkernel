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

//! One transfer from a queue into what the host armed.

use std::collections::VecDeque;

/// The next transfer, which must fit what was armed: a device that sends
/// more babbles (USB 2.0, 8.4.2). A transfer given ends the host's quiet.
pub fn give(
    q: &mut VecDeque<Vec<u8>>,
    idle: &mut u32,
    out: &mut [u8],
) -> Result<Option<usize>, i32> {
    let Some(t) = q.pop_front() else { return Ok(None) };
    assert!(t.len() <= out.len(), "a transfer longer than the armed length is babble");
    out[..t.len()].copy_from_slice(&t);
    *idle = 0;
    Ok(Some(t.len()))
}
