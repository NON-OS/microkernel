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

//! Finding the USB hosts (usb_hosts.rs, fastest first): those not yet
//! registered are looked up at every step while the search lasts, so a
//! device is found as soon as its host registers, and after it every few
//! seconds, as the ports are. They were looked up every 5 ms for as long as
//! none registered, on a machine with no USB at all for the boot.

use super::report::Report;
use super::scanner::Scanner;
use super::searching::RESCAN_MS;
use crate::clock::Deadline;
use crate::xhci::host;

impl Scanner {
    /// Whether any host has registered; the search restarts when the first
    /// does.
    pub(super) fn found(&mut self) -> bool {
        if !(self.searching || self.look.expired()) {
            return self.known > 0;
        }
        let known = self.hosts.find(host);
        match (self.known, known) {
            (_, 0) => {
                self.report = Report::new();
                self.look = Deadline::after_ms(RESCAN_MS);
            }
            (0, _) if self.searching => self.restart(),
            _ => {}
        }
        self.known = known;
        known > 0
    }
}
