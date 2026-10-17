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
//! The host's display events (G-77): a window resized, or a display turned
//! on or off, raises the display bit in the device's events word. STATE
//! reads it; when set, the display info is asked again, each head takes the
//! size the host now wants, the bit is cleared and the generation moves, so
//! the compositor looks again (and follows the host where it does). An info
//! the host does not answer leaves the bit set, to be read again.

use super::gpu::Gpu;
use crate::constants::{GPU_CFG_EVENTS_CLEAR, GPU_CFG_EVENTS_READ, GPU_EVENT_DISPLAY};
use crate::device::cmd::get_display_info;

impl Gpu {
    pub fn events(&mut self) {
        // SAFETY: the registers map the device configuration (bring-up).
        let raised = unsafe { self.q.regs.config_r32(GPU_CFG_EVENTS_READ) } & GPU_EVENT_DISPLAY;
        if raised == 0 {
            return;
        }
        let Ok(info) = get_display_info(&self.q, self.fences.issue()) else { return };
        for h in self.heads.iter_mut().flatten() {
            let s = info.scanouts[h.scanout as usize];
            if s.enabled != 0 && s.width > 0 && s.height > 0 {
                h.hint = (s.width, s.height);
            }
        }
        // SAFETY: as above.
        unsafe { self.q.regs.config_w32(GPU_CFG_EVENTS_CLEAR, raised) };
        self.generation = self.generation.wrapping_add(1);
    }
}
