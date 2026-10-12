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

//! After the window, and what every pass needs: the hosts that have
//! registered, the first of them restarting the window, and the medium
//! while the search is open.

use nonos_libc::Deadline;

use super::medium::Medium;
use super::pass::{pass, NOT_OURS};
use super::report::Report;
use super::scanner::{Scanner, WINDOW_MS};
use crate::state::State;
use crate::xhci::{connected_ports, host};

impl Scanner {
    /// After the window: every port that failed is given its tries again,
    /// and one pass looks at every port not of another class on every host.
    pub fn again(&mut self, state: &mut State) -> Medium {
        if !self.found() {
            return Medium::Absent;
        }
        self.hosts.each(|_, tries| {
            for t in tries.iter_mut().filter(|t| **t != NOT_OURS) {
                *t = 0;
            }
        });
        let report = &mut self.report;
        let disk = self.hosts.first(|xhci, tries| {
            let ports = connected_ports(xhci).ok()?;
            pass(xhci, ports, tries, state, report).0
        });
        disk.map_or(Medium::Absent, Medium::Ready)
    }

    /// Whether any host has registered; one that registered since the last
    /// pass is searched from this one, and the first restarts the window.
    pub(super) fn found(&mut self) -> bool {
        let known = self.hosts.find(host);
        if known == 0 {
            self.report = Report::new();
            return false;
        }
        if self.known == 0 {
            self.window = Deadline::after_ms(WINDOW_MS);
        }
        self.known = known;
        true
    }

    pub(super) fn undecided(&self) -> Medium {
        if self.window.expired() {
            Medium::Absent
        } else {
            Medium::Scanning
        }
    }
}
