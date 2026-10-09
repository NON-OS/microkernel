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

//! The bits of the IOMMU Control register (MMIO 0x18) a unit left running
//! by firmware may have set, and the value that stops it. Pure.

pub(super) const CONTROL: usize = 0x18;

pub(super) const IOMMU_EN: u64 = 1 << 0;
pub(super) const EVENT_LOG_EN: u64 = 1 << 2;
const EVENT_INT_EN: u64 = 1 << 3;
const COM_WAIT_INT_EN: u64 = 1 << 4;
pub(super) const CMD_BUF_EN: u64 = 1 << 12;
const PPR_LOG_EN: u64 = 1 << 13;
const PPR_INT_EN: u64 = 1 << 14;
const PPR_EN: u64 = 1 << 15;
const GT_EN: u64 = 1 << 16;
const GA_EN: u64 = 1 << 17;
const XT_EN: u64 = 1 << 50;

/// Everything that makes the unit translate, log or interrupt. The rest
/// (coherency, isochrony, timeouts) is left as firmware set it.
const RUNNING: u64 = IOMMU_EN
    | EVENT_LOG_EN
    | EVENT_INT_EN
    | COM_WAIT_INT_EN
    | CMD_BUF_EN
    | PPR_LOG_EN
    | PPR_INT_EN
    | PPR_EN
    | GT_EN
    | GA_EN
    | XT_EN;

/// The control value with the unit stopped, or `None` if it is not running.
pub(super) const fn stopped(control: u64) -> Option<u64> {
    if control & IOMMU_EN == 0 {
        return None;
    }
    Some(idle(control))
}

/// The control value with everything that translates, logs or interrupts
/// cleared, whether or not the unit was running: firmware can leave the
/// command buffer or the event log enabled on a unit it turned off.
pub(super) const fn idle(control: u64) -> u64 {
    control & !RUNNING
}
