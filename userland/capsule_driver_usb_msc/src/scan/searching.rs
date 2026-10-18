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

//! When the search runs. Nothing tells the kernel that a USB mass-storage
//! device is plugged in until the controller driver has enumerated its
//! ports, so this driver starts wherever an xHCI controller is and decides,
//! within a window from when the controller is found, whether it has a
//! device. After it, ports are still looked at every few seconds, so a late
//! stick is still taken.

use super::medium::Medium;
use super::scanner::Scanner;
use crate::clock::Deadline;

/// How long driver.xhci0 is waited for, and a device after it is found;
/// how long a port is given to report its device; how often ports are
/// listed while the search lasts, and after it.
pub(super) const XHCI_WAIT_MS: u64 = 30_000;
const WINDOW_MS: u64 = 10_000;
const SETTLE_MS: u64 = 1_500;
pub(super) const SCAN_POLL_MS: u64 = 5;
pub(super) const RESCAN_MS: u64 = 5_000;

impl Scanner {
    /// The search starts over: the controller came, or a device was let go.
    pub(super) fn restart(&mut self) {
        (self.searching, self.medium) = (true, Medium::Scanning);
        (self.window, self.settle) = (Deadline::after_ms(WINDOW_MS), Deadline::after_ms(SETTLE_MS));
        self.look = Deadline::after_ms(0);
    }

    /// Whether the search is over: its window closed, or every port is
    /// decided and given its time to report, with no device being asked. A
    /// device asked when the window closes is asked to the end, thirty
    /// seconds at most (`asking`), so a stick slow to spin up is still found.
    pub(super) fn decide(&mut self) {
        if !self.searching || self.asking.is_some() {
            return;
        }
        if self.window.expired() || (!self.open && self.settle.expired()) {
            self.searching = false;
            if matches!(self.medium, Medium::Scanning) {
                self.medium = Medium::Absent;
            }
        }
    }
}
