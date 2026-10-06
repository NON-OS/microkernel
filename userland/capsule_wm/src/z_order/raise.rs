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

use super::ZStack;
use crate::window::WindowTable;

/// Put a window on top of the stack. Some(true) when that changed the order,
/// that is when another window sat above it; Some(false) when it was already
/// on top and keeps its z; None when the table has no such window.
///
/// Every raise goes through here so the caller learns whether the order moved,
/// and tells the compositor exactly then: the compositor stacks layers by the
/// raises it is told about, and a raise it never heard of is a window drawn
/// under the one the hit test gives the next click to.
pub fn raise(
    windows: &mut WindowTable,
    z: &mut ZStack,
    owner_pid: u32,
    window_id: u32,
) -> Option<bool> {
    let current = windows.find(owner_pid, window_id)?.z;
    let covered = windows.windows().any(|w| !w.matches(owner_pid, window_id) && w.z > current);
    if !covered {
        return Some(false);
    }
    let top = z.allocate();
    windows.find_mut(owner_pid, window_id)?.z = top;
    Some(true)
}
