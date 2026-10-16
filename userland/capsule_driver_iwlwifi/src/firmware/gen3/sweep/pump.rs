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

//! One pass of received buffers, and what it left the sweep at.

use core::cell::Cell;

use super::super::dev::Dev;
use super::super::region::Region;
use super::super::rx_frame::RxFrame;
use super::kinds::Tick;
use super::state::{route, Sweep};
use crate::regs::Mmio;

impl Sweep {
    /// Take one pass of received buffers at `now_ms`. Without a sweep in
    /// flight the buffers are still drained (and restocked).
    pub fn pump<M: Mmio, R: Region + ?Sized>(
        &mut self,
        dev: &mut Dev<'_, M, R>,
        now_ms: u64,
        on_frame: &mut dyn FnMut(RxFrame),
    ) -> Tick {
        let status = Cell::new(None);
        let frames = &mut self.frames;
        dev.drain(&mut |p| route(p, on_frame, frames, &status));
        let Some(started) = self.started_ms else {
            return if dev.error_cause() { Tick::Failed } else { Tick::Idle };
        };
        if let Some(s) = status.get() {
            self.started_ms = None;
            self.completed = self.completed.saturating_add(1);
            return Tick::Completed(s);
        }
        if dev.error_cause() {
            self.started_ms = None;
            return Tick::Failed;
        }
        if now_ms.saturating_sub(started) > self.budget_ms {
            self.started_ms = None;
            self.stalled = self.stalled.saturating_add(1);
            return Tick::Stalled;
        }
        Tick::Running
    }
}
