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

//! The background scan the serving loop runs between requests: one passive
//! firmware scan at a time, pumped a pass of received buffers per call, so a
//! scan request is answered at once from what was heard instead of blocking
//! the loop for a whole sweep. Every received management frame goes to the
//! caller as it is drained. The completion carrying this scan's uid ends the
//! sweep; a sweep that runs past its channel budget is given up as stalled,
//! and a firmware or hardware error cause ends it as failed, so a firmware
//! that never completes cannot hold the loop. The caller passes the time, so
//! the proofs drive it along any timeline. A join stops a sweep early
//! (`abort`): the firmware then completes it with the aborted status, which
//! ends it as any completion does.

use core::cell::Cell;

use super::bringup::is_legacy;
use super::cmds::{LONG_GROUP, REPLY_RX_MPDU_CMD, SCAN_COMPLETE_UMAC, SCAN_REQ_UMAC};
use super::dev::{Dev, Flow, WaitError};
use super::packet::Packet;
use super::region::{Clock, Region};
use super::rx_frame::{parse as parse_frame, RxFrame};
use super::scan::{complete, request, SCAN_UID};
use super::station::abort::scan_abort;
use super::station::ids::SCAN_ABORT_UMAC;
use crate::regs::Mmio;

/// Per-channel allowance: the 110 ms passive dwell and the switch.
const PER_CHANNEL_MS: u32 = 150;
const MARGIN_MS: u32 = 2000;

/// How long a sweep over `channels` channels may run before it is given up.
pub fn budget_ms(channels: usize) -> u32 {
    let n = u32::try_from(channels).unwrap_or(u32::MAX);
    PER_CHANNEL_MS.saturating_mul(n).saturating_add(MARGIN_MS)
}

/// Why a sweep could not start.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ScanError {
    /// No channels, or more than one request carries.
    NoChannels,
    /// The scan request was not answered (or the firmware failed meanwhile).
    Request(WaitError),
}

/// What one call left the sweep at.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tick {
    /// No sweep is in flight (the buffers were still drained).
    Idle,
    /// The sweep is still running.
    Running,
    /// The firmware reported this scan complete with this status.
    Completed(u8),
    /// The sweep ran past its budget with no completion.
    Stalled,
    /// The firmware or hardware raised its error cause.
    Failed,
}

/// One background sweep and its running counts.
#[derive(Default)]
pub struct Sweep {
    started_ms: Option<u64>,
    budget_ms: u64,
    /// Management frames handed up, across all sweeps.
    pub frames: u32,
    /// Sweeps the firmware completed.
    pub completed: u32,
    /// Sweeps given up with no completion.
    pub stalled: u32,
}

// Send each received management frame up and note this scan's completion.
fn route(p: &Packet<'_>, on_frame: &mut dyn FnMut(RxFrame), frames: &mut u32, status: &Cell<Option<u8>>) -> Flow {
    if is_legacy(p, REPLY_RX_MPDU_CMD) {
        if let Some(f) = parse_frame(p.payload) {
            *frames = frames.saturating_add(1);
            on_frame(f);
        }
    } else if is_legacy(p, SCAN_COMPLETE_UMAC) {
        if let Some((uid, s)) = complete(p.payload) {
            if uid == SCAN_UID {
                status.set(Some(s));
                return Flow::Done;
            }
        }
    }
    Flow::Continue
}

impl Sweep {
    pub fn new() -> Self {
        Self::default()
    }

    /// A sweep is in flight.
    pub fn running(&self) -> bool {
        self.started_ms.is_some()
    }

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
        let req = request(channels).ok_or(ScanError::NoChannels)?;
        let status = Cell::new(None);
        let frames = &mut self.frames;
        dev.command(c, (LONG_GROUP, SCAN_REQ_UMAC), &req, &mut |_| {}, &mut |p| {
            let _ = route(p, on_frame, frames, &status);
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
