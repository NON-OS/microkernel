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

//! The search for the device, one pass over the root ports at a time.

use nonos_libc::Deadline;

use super::book::Book;
use super::bound::Bound;
use super::probe::{probe, Probe};
use crate::run::BindFn;
use crate::xhci::{connected_ports, lookup};

/// How often every port decided against is tried again, for a phone whose
/// switch to tethering the connect-change bit did not show.
const RETRY_ALL_MS: u64 = 60_000;

pub struct Scanner {
    xhci: Option<u32>,
    book: Book,
    retry_all: Deadline,
}

impl Scanner {
    pub(crate) fn new() -> Self {
        let retry_all = Deadline::after_ms(RETRY_ALL_MS);
        Self { xhci: None, book: Book::default(), retry_all }
    }

    /// The device, when a port this pass holds one this driver binds.
    pub fn pass<N>(&mut self, tag: &[u8], bind: BindFn<N>) -> Option<Bound<N>> {
        let xhci = match self.xhci {
            Some(x) => x,
            None => *self.xhci.insert(lookup()?),
        };
        if self.retry_all.expired() {
            self.book.retry_all();
            self.retry_all = Deadline::after_ms(RETRY_ALL_MS);
        }
        let ports = connected_ports(xhci).ok()?;
        for port in self.book.plan(&ports) {
            match probe(xhci, port, tag, bind) {
                Probe::Ours(bound) => return Some(bound),
                Probe::NotOurs => self.book.not_ours(port),
                Probe::Busy => {}
                Probe::Failed => self.book.failed(port),
            }
        }
        None
    }

    /// A bound device stopped answering: its port counts a failure, so one
    /// that keeps failing is left after `TRIES`.
    pub fn lost(&mut self, port: u8) {
        self.book.failed(port);
    }
}
