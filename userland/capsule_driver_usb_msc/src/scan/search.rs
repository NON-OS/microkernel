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

//! One step of the search: the next command to the device being asked, or,
//! with none, a look at every host's ports when one is due (`look`) and the
//! bind of the first that may be asked.

use super::probe::{bind, Probe};
use super::report::{Report, Stage};
use super::scanner::Scanner;
use super::searching::{RESCAN_MS, SCAN_POLL_MS};
use crate::clock::Deadline;
use crate::state::State;

impl Scanner {
    pub(super) fn search(&mut self, state: &mut State) {
        if let Some(asking) = self.asking.take() {
            let xhci = asking.disk.xhci;
            self.asked(xhci, asking.step(state, self.served));
        } else if self.look.expired() {
            self.look = Deadline::after_ms(if self.searching { SCAN_POLL_MS } else { RESCAN_MS });
            self.look_at(state);
        }
        self.decide();
    }

    /// List the ports, and bind the first that may be asked.
    fn look_at(&mut self, state: &mut State) {
        let look = self.look();
        (self.left_on, self.open) = (look.left, look.open);
        match look.failed {
            Some(e) if look.lists.is_empty() => {
                return self.report = Report::at(Stage::NoXhci, 0, e)
            }
            _ if look.connected == 0 => self.report = Report::at(Stage::NoPort, 0, 0),
            _ => {}
        }
        match look.next.map(|(xhci, port)| (xhci, port, bind(xhci, port, state))) {
            None => {}
            Some((_, _, Probe::Bound(asking))) => self.asking = Some(asking),
            Some((xhci, port, Probe::NotOurs(class))) => {
                self.ports_of(xhci, |p| p.set(port, true));
                self.report = Report { aux: class, ..Report::at(Stage::NotStorage, port, 0) };
            }
            Some((_, port, Probe::Busy)) => self.report = Report::at(Stage::Busy, port, 0),
            Some((xhci, port, Probe::Failed(stage, e))) => {
                self.ports_of(xhci, |p| p.failed(port, false));
                self.report = Report::at(stage, port, e);
            }
        }
        self.report.connected = look.connected.min(u8::MAX as usize) as u8;
        self.report.closed = self.closed(&look.lists).min(u8::MAX as usize) as u8;
    }
}
