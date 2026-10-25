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
//! The host's display events: a window resized, or a display turned
//! on or off, raises the display bit in the device's events word. The
//! receive loop reads it on every turn, and STATE too; when set, the
//! display info is asked again and each head follows it (`plug`), the bit
//! is cleared, and on any change the generation moves so the compositor
//! looks again. An info the host does not answer leaves the bit set, to be
//! read again.

use super::gpu::Gpu;
use crate::constants::{GPU_CFG_EVENTS_CLEAR, GPU_CFG_EVENTS_READ, GPU_EVENT_DISPLAY};
use crate::device::cmd::get_display_info;
use crate::state::Health;

impl Gpu {
    /// Whether an output changed: its size wanted, or turned off or back.
    pub fn events(&mut self) -> bool {
        let raised = self.q.regs.config_r32(GPU_CFG_EVENTS_READ) & GPU_EVENT_DISPLAY;
        // A Down device's queue takes no command: its outputs are LOST.
        if raised == 0 || self.health == Health::Down {
            return false;
        }
        let Ok(info) = get_display_info(&self.q, self.fences.issue()) else { return false };
        let changed = self.follow(&info);
        self.q.regs.config_w32(GPU_CFG_EVENTS_CLEAR, raised);
        if changed {
            self.generation = self.generation.wrapping_add(1);
        }
        changed
    }
}
