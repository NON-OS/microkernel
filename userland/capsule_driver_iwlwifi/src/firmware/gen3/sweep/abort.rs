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

//! Stopping the sweep in flight early.

use super::super::dev::{Dev, WaitError};
use super::super::region::{Clock, Region};
use super::super::rx_frame::RxFrame;
use super::state::Sweep;
use crate::regs::Mmio;

impl Sweep {
    /// Ask the firmware to stop the sweep in flight. Frames and the
    /// completion arriving meanwhile are handled as `pump` would; the sweep
    /// ends when its completion comes (here or in a later `pump`).
    pub fn abort<M: Mmio, R: Region + ?Sized, C: Clock>(
        &mut self,
        dev: &mut Dev<'_, M, R>,
        c: &mut C,
        on_frame: &mut dyn FnMut(RxFrame),
    ) -> Result<(), WaitError> {
        if !self.running() {
            return Ok(());
        }
        if self.send_abort(dev, c, on_frame, self.uid)? {
            self.started_ms = None;
            self.completed = self.completed.saturating_add(1);
        }
        Ok(())
    }
}
