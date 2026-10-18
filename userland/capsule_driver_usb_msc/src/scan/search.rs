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
//! with none, a look at the ports when one is due and the bind of the first
//! that may be asked: the port of one let go and still on it (`release`),
//! while it is listed, else any.

use super::probe::{bind, Probe};
use super::report::{Report, Stage};
use super::scanner::Scanner;
use super::searching::{RESCAN_MS, SCAN_POLL_MS};
use crate::clock::Deadline;
use crate::state::State;
use crate::xhci::connected_ports;

impl Scanner {
    pub(super) fn search(&mut self, xhci: u32, state: &mut State) {
        if let Some(asking) = self.asking.take() {
            self.asked(xhci, asking.step(state, self.served));
        } else if self.look.expired() {
            self.look = Deadline::after_ms(if self.searching { SCAN_POLL_MS } else { RESCAN_MS });
            self.look_at(xhci, state);
        }
        self.decide();
    }

    /// List the ports, and bind the first that may be asked.
    fn look_at(&mut self, xhci: u32, state: &mut State) {
        let listed = match connected_ports(xhci) {
            Ok(listed) => listed,
            Err(e) => {
                self.report = Report::at(Stage::NoXhci, 0, e);
                return;
            }
        };
        self.ports.forget_gone(&listed);
        let stayed = listed.iter().find(|p| Some(p.id) == self.left_on);
        self.left_on = stayed.map(|p| p.id);
        let (next, open) = self.ports.next(stayed.map_or(&listed[..], core::slice::from_ref));
        self.open = open;
        if listed.is_empty() {
            self.report = Report::at(Stage::NoPort, 0, 0);
        }
        match next.map(|port| (port, bind(xhci, port, state))) {
            None => {}
            Some((_, Probe::Bound(asking))) => self.asking = Some(asking),
            Some((port, Probe::NotOurs(class))) => {
                self.ports.set(port, true);
                self.report = Report { aux: class, ..Report::at(Stage::NotStorage, port, 0) };
            }
            Some((port, Probe::Busy)) => self.report = Report::at(Stage::Busy, port, 0),
            Some((port, Probe::Failed(stage, e))) => {
                self.ports.failed(port, false);
                self.report = Report::at(stage, port, e);
            }
        }
        self.report.connected = listed.len().min(u8::MAX as usize) as u8;
        self.report.closed = self.ports.closed(&listed);
    }
}
