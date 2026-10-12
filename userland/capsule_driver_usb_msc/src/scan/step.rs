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
//! One pass over every registered host's ports, fastest host first.

use nonos_libc::Deadline;

use super::medium::Medium;
use super::pass::pass;
use super::report::{Report, Stage};
use super::scanner::{Scanner, SETTLE_MS};
use crate::state::State;
use crate::xhci::connected_ports;

impl Scanner {
    /// One pass over the ports. `Scanning` until the device is bound or
    /// every connected port is decided, or the window closes.
    pub fn step(&mut self, state: &mut State) -> Medium {
        if !self.found() {
            return self.undecided();
        }
        let settle = *self.settle.get_or_insert_with(|| Deadline::after_ms(SETTLE_MS));
        let (report, mut open) = (&mut self.report, false);
        let disk = self.hosts.first(|xhci, tries| match connected_ports(xhci) {
            Ok(ports) => {
                let (disk, still) = pass(xhci, ports, tries, state, report);
                open |= still;
                disk
            }
            Err(e) => {
                *report = Report::at(Stage::NoXhci, 0, e);
                open = true;
                None
            }
        });
        if let Some(disk) = disk {
            return Medium::Ready(disk);
        }
        if open || !settle.expired() {
            return self.undecided();
        }
        Medium::Absent
    }
}
