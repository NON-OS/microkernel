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

use alloc::vec::Vec;

use crate::kernel_core::surface_registry::attach_map::snapshot::records_of;

use super::frames_of::frames_of;
use super::window::Window;

/* Windows `pid` attached from other owners that overlap [addr, end). */
pub(super) fn foreign_windows(pid: u32, addr: u64, end: u64, out: &mut Vec<Window>) {
    for (handle, base) in records_of(pid) {
        let Some((owner, frames)) = frames_of(handle) else { continue };
        let win_end = base.saturating_add(frames.len() as u64 * 4096);
        if owner != pid && base < end && addr < win_end {
            out.push(Window::held(base, frames));
        }
    }
}
