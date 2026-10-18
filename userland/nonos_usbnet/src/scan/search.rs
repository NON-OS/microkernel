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

//! One pass over the root ports: each port the book plans is probed with
//! the driver's binding until one holds its device.

use super::bound::Bound;
use super::probe::{probe, Probe};
use super::scanner::Scanner;
use crate::bind::Bind;
use crate::found::Found;
use crate::xhci::{connected_ports, lookup, XhciBus};

impl Scanner {
    pub(super) fn search<N>(
        &mut self,
        tag: &[u8],
        bind: impl Fn(XhciBus, &Found) -> Bind<N>,
    ) -> Option<Bound<N>> {
        self.xhci = self.xhci.or_else(lookup);
        let xhci = self.xhci?;
        if self.backoff.due() {
            self.book.retry_all();
        }
        let ports = connected_ports(xhci).ok()?;
        for port in self.book.plan(&ports) {
            match probe(xhci, port, tag, &bind) {
                Probe::Ours(bound) => {
                    self.backoff.reset();
                    return Some(bound);
                }
                Probe::NotOurs => self.book.not_ours(port),
                Probe::Busy => {}
                Probe::Failed => self.book.failed(port),
            }
        }
        None
    }
}
