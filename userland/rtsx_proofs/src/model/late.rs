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

//! A late completion: whatever command buffer the reader still holds,
//! stalled or told to stop, finishes at the device's next turn at the
//! broker (an unmap, or the release once bus mastering is off) and writes
//! its results to DMA memory then, as a chip that took its time would.

use nonos_libc::{note, set_device_hook};

use super::reader::with;

pub fn complete_late_at_teardown() {
    set_device_hook(|| {
        with(|r| {
            if let Some(run) = r.run.take() {
                note("late completion");
                r.knobs.stall_cmd = false;
                r.resume(run);
            }
        })
    });
}
