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

use core::cell::Cell;

use super::super::cmds::LONG_GROUP;
use super::super::dev::{Dev, WaitError};
use super::super::region::{Clock, Region};
use super::super::rx_frame::RxFrame;
use super::super::station::abort::scan_abort;
use super::super::station::ids::SCAN_ABORT_UMAC;
use super::state::{route, Sweep};
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
        let status = Cell::new(None);
        let frames = &mut self.frames;
        dev.command(c, (LONG_GROUP, SCAN_ABORT_UMAC), &scan_abort(), &mut |_| {}, &mut |p| {
            let _ = route(p, on_frame, frames, &status);
        })?;
        if status.get().is_some() {
            self.started_ms = None;
            self.completed = self.completed.saturating_add(1);
        }
        Ok(())
    }
}
