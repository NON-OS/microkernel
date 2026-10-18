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

//! Starting a sweep: the scan request sent and answered.

use core::cell::Cell;

use super::super::cmds::{LONG_GROUP, SCAN_REQ_UMAC};
use super::super::dev::Dev;
use super::super::region::{Clock, Region};
use super::super::rx_frame::RxFrame;
use super::super::scan::{request, SCAN_UIDS};
use super::kinds::{budget_ms, ScanError, Tick};
use super::route::route;
use super::state::Sweep;
use crate::regs::Mmio;

impl Sweep {
    /// Start a sweep over `channels` at `now_ms`: send the scan request and
    /// wait for its reply. Frames (and even the completion) arriving before
    /// the reply are handled as `pump` would.
    pub fn start<M: Mmio, R: Region + ?Sized, C: Clock>(
        &mut self,
        dev: &mut Dev<'_, M, R>,
        c: &mut C,
        channels: &[u8],
        now_ms: u64,
        on_frame: &mut dyn FnMut(RxFrame),
    ) -> Result<Tick, ScanError> {
        let uid = self.next_uid;
        let req = request(channels, uid).ok_or(ScanError::NoChannels)?;
        (self.uid, self.next_uid) = (uid, (uid + 1) % SCAN_UIDS);
        let status = Cell::new(None);
        let frames = &mut self.frames;
        dev.command(c, (LONG_GROUP, SCAN_REQ_UMAC), &req, &mut |_| {}, &mut |p| {
            let _ = route(p, on_frame, frames, &status, uid);
        })
        .map_err(ScanError::Request)?;
        if let Some(s) = status.get() {
            self.completed = self.completed.saturating_add(1);
            return Ok(Tick::Completed(s));
        }
        self.started_ms = Some(now_ms);
        self.budget_ms = u64::from(budget_ms(channels.len()));
        Ok(Tick::Running)
    }
}
