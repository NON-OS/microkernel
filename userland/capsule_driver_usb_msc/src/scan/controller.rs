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

//! Finding driver.xhci0: at every step while the search lasts, so a device
//! is found as soon as the controller registers, and after it every few
//! seconds, as the ports are. It was looked up every 5 ms for as long as no
//! controller registered, on a machine with no USB at all for the boot.

use super::report::Report;
use super::scanner::Scanner;
use super::searching::RESCAN_MS;
use crate::clock::Deadline;
use crate::xhci::lookup;

impl Scanner {
    /// driver.xhci0, once it has registered; the search restarts then.
    pub(super) fn found(&mut self) -> Option<u32> {
        if self.xhci.is_some() || !(self.searching || self.look.expired()) {
            return self.xhci;
        }
        self.xhci = lookup();
        match self.xhci {
            None => {
                self.report = Report::new();
                self.look = Deadline::after_ms(RESCAN_MS);
            }
            Some(_) if self.searching => self.restart(),
            Some(_) => {}
        }
        self.xhci
    }
}
