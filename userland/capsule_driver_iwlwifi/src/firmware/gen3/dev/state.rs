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

//! The transport's state, and its error cause.

use super::super::cmdq::CmdQueue;
use super::super::plan::RB_SIZE;
use super::super::region::Region;
use super::super::regs::{
    CSR_INT, CSR_MSIX_HW_INT_CAUSES_AD, INT_BIT_HW_ERR, INT_BIT_SW_ERR, MSIX_HW_HW_ERR,
    MSIX_HW_SW_ERR,
};
use super::super::rxq::RxQueue;
use crate::regs::Mmio;

pub struct Dev<'a, M: Mmio, R: Region + ?Sized> {
    pub m: &'a M,
    pub ctrl: &'a R,
    pub rbs: &'a R,
    pub rxq: RxQueue,
    pub cmdq: CmdQueue,
    /// Buffers the device named that were not posted.
    pub bad_ids: u32,
    pub(super) buf: [u8; RB_SIZE],
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
}
