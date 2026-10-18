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

//! SCAN_ABORT for a sweep's uid: the one in flight, or one given up as
//! stalled, so the firmware frees its slot.

use core::cell::Cell;

use super::super::cmds::LONG_GROUP;
use super::super::dev::{Dev, WaitError};
use super::super::region::{Clock, Region};
use super::super::rx_frame::RxFrame;
use super::super::station::abort::scan_abort;
use super::super::station::ids::SCAN_ABORT_UMAC;
use super::route::route;
use super::state::Sweep;
use crate::regs::Mmio;

impl Sweep {
    /// Stop a sweep given up as stalled, so the firmware frees its slot;
    /// its completion, if one comes, carries its own uid and ends nothing.
    pub fn abort_stalled<M: Mmio, R: Region + ?Sized, C: Clock>(
        &mut self,
        dev: &mut Dev<'_, M, R>,
        c: &mut C,
        on_frame: &mut dyn FnMut(RxFrame),
    ) -> Result<(), WaitError> {
        match self.stalled_uid.take() {
            Some(uid) => self.send_abort(dev, c, on_frame, uid).map(|_| ()),
            None => Ok(()),
        }
    }

    // SCAN_ABORT for `uid`; whether its completion came meanwhile.
    pub(super) fn send_abort<M: Mmio, R: Region + ?Sized, C: Clock>(
        &mut self,
        dev: &mut Dev<'_, M, R>,
        c: &mut C,
        on_frame: &mut dyn FnMut(RxFrame),
        uid: u32,
    ) -> Result<bool, WaitError> {
        let status = Cell::new(None);
        let frames = &mut self.frames;
        dev.command(c, (LONG_GROUP, SCAN_ABORT_UMAC), &scan_abort(uid), &mut |_| {}, &mut |p| {
            let _ = route(p, on_frame, frames, &status, uid);
        })?;
        Ok(status.get().is_some())
    }
}
