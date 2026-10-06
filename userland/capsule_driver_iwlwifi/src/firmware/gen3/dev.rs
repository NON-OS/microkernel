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

//! The running transport: registers, the control and receive-buffer regions,
//! and both queues. It takes received packets in order, hands each to the
//! caller's handler, and runs a host command to its reply. Every wait is one
//! bounded clock wait, and ends early on a firmware or hardware error cause.

use super::cmdq::CmdQueue;
use super::packet::{parse as parse_packet, Packet};
use super::plan::RB_SIZE;
use super::region::{Clock, Region};
use super::regs::{
    CSR_INT, CSR_MSIX_HW_INT_CAUSES_AD, INT_BIT_HW_ERR, INT_BIT_SW_ERR, MSIX_HW_HW_ERR,
    MSIX_HW_SW_ERR,
};
use super::rxq::{RxQueue, Taken};
use crate::regs::Mmio;

/// `HOST_COMPLETE_TIMEOUT`: how long a command may take to answer.
pub const COMMAND_MS: u32 = 2000;
/// Received buffers taken per pass.
const PASS: usize = 32;
/// `TX_CMD`: in the legacy group, a frame's transmit response. It carries
/// the frame's queue and index as its sequence and no `SEQ_RX_FRAME`, so it
/// can look like a command's reply; Linux never takes it for one
/// (`no_reclaim_cmds` in `iwl_pcie_rx_handle_rb`).
const TX_RESPONSE: u8 = 0x1C;

/// Why a wait ended without what it waited for.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum WaitError {
    TimedOut,
    /// The firmware or the hardware raised its error cause.
    FirmwareError,
    /// The command did not fit the command buffer.
    TooLarge,
}

/// What a packet handler wants next.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Flow {
    Continue,
    Done,
}

pub struct Dev<'a, M: Mmio, R: Region + ?Sized> {
    pub m: &'a M,
    pub ctrl: &'a R,
    pub rbs: &'a R,
    pub rxq: RxQueue,
    pub cmdq: CmdQueue,
    /// Buffers the device named that were not posted.
    pub bad_ids: u32,
    buf: [u8; RB_SIZE],
    /// The version of the platform NVM section the firmware was given, if any.
    pub pnvm: Option<u32>,
}

impl<'a, M: Mmio, R: Region + ?Sized> Dev<'a, M, R> {
    pub fn new(m: &'a M, ctrl: &'a R, rbs: &'a R) -> Self {
        Self {
            m,
            ctrl,
            rbs,
            rxq: RxQueue::new(),
            cmdq: CmdQueue::new(),
            bad_ids: 0,
            buf: [0; RB_SIZE],
            pnvm: None,
        }
    }

    /// The firmware or hardware error cause is set.
    pub fn error_cause(&self) -> bool {
        self.m.read32(CSR_INT) & (INT_BIT_SW_ERR | INT_BIT_HW_ERR) != 0
            || self.m.read32(CSR_MSIX_HW_INT_CAUSES_AD) & (MSIX_HW_SW_ERR | MSIX_HW_HW_ERR) != 0
    }

    /// Take up to a pass of received packets, giving each whole one to
    /// `handle`, then restock the device. `Done` if `handle` said so.
    pub fn drain(&mut self, handle: &mut dyn FnMut(&Packet<'_>) -> Flow) -> Flow {
        let mut flow = Flow::Continue;
        for _ in 0..PASS {
            match self.rxq.take(self.ctrl, self.rbs, &mut self.buf) {
                None => break,
                Some(Taken::BadId) => self.bad_ids = self.bad_ids.saturating_add(1),
                Some(Taken::Fragment) => {}
                Some(Taken::Buffer) => {
                    if let Some(p) = parse_packet(&self.buf) {
                        if handle(&p) == Flow::Done {
                            flow = Flow::Done;
                            break;
                        }
                    }
                }
            }
        }
        self.rxq.kick(self.m);
        flow
    }

    /// Wait up to `ms` for `handle` to say `Done`.
    pub fn wait<C: Clock>(
        &mut self,
        c: &mut C,
        ms: u32,
        handle: &mut dyn FnMut(&Packet<'_>) -> Flow,
    ) -> Result<(), WaitError> {
        let mut outcome = None;
        c.poll_for(ms, &mut || {
            if self.drain(handle) == Flow::Done {
                outcome = Some(Ok(()));
            } else if self.error_cause() {
                outcome = Some(Err(WaitError::FirmwareError));
            }
            outcome.is_some()
        });
        outcome.unwrap_or(Err(WaitError::TimedOut))
    }

    /// Send a command and wait for its reply; `reply` gets the reply payload.
    /// Notifications arriving meanwhile go to `other`.
    pub fn command<C: Clock>(
        &mut self,
        c: &mut C,
        (group, cmd): (u8, u8),
        payload: &[u8],
        reply: &mut dyn FnMut(&[u8]),
        other: &mut dyn FnMut(&Packet<'_>),
    ) -> Result<(), WaitError> {
        let seq =
            self.cmdq.send(self.m, self.ctrl, group, cmd, 0, payload).ok_or(WaitError::TooLarge)?;
        self.wait(c, COMMAND_MS, &mut |p| {
            if p.is_reply() && p.sequence == seq && !(p.group == 0 && p.cmd == TX_RESPONSE) {
                reply(p.payload);
                Flow::Done
            } else {
                other(p);
                Flow::Continue
            }
        })
    }
}
