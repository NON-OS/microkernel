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

//! Scanning, passive only: the controller listens and never sends a scan
//! request, so looking for devices puts nothing of ours on the air. The
//! reports arrive as LE Advertising Report events through `next_event`.

use super::state::Host;
use crate::error::HciError;
use crate::transport::Transport;
use nonos_bt_wire::hci::opcode as op;

/// Interval and window in 0.625 ms units, each 0x0004 to 0x4000 with the
/// window no longer than the interval (Vol 4, Part E, 7.8.10).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Scan {
    pub interval: u16,
    pub window: u16,
}

/// Listening 30 ms of every 60 ms is half the radio's time, enough to hear
/// a device that advertises every 100 ms within a few intervals.
pub const DEFAULT_SCAN: Scan = Scan { interval: 0x0060, window: 0x0030 };

const PASSIVE: u8 = 0x00;
const OWN_PUBLIC: u8 = 0x00;
const ACCEPT_ALL: u8 = 0x00;

impl<T: Transport> Host<T> {
    /// Start a passive scan with duplicate reports filtered by the
    /// controller (Vol 4, Part E, 7.8.11).
    pub fn scan_passive(&mut self, s: Scan) -> Result<(), HciError> {
        let valid = 0x0004..=0x4000;
        if !valid.contains(&s.interval) || !valid.contains(&s.window) || s.window > s.interval {
            return Err(HciError::Wire(nonos_bt_wire::WireError::Reserved));
        }
        let [i0, i1] = s.interval.to_le_bytes();
        let [w0, w1] = s.window.to_le_bytes();
        let params = [PASSIVE, i0, i1, w0, w1, OWN_PUBLIC, ACCEPT_ALL];
        self.command(op::LE_SET_SCAN_PARAMETERS, &params)?;
        self.command(op::LE_SET_SCAN_ENABLE, &[1, 1]).map(|_| ())
    }

    pub fn scan_stop(&mut self) -> Result<(), HciError> {
        self.command(op::LE_SET_SCAN_ENABLE, &[0, 0]).map(|_| ())
    }
}
