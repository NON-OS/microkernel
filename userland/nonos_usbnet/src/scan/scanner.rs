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

use super::backoff::Backoff;
use super::book::Book;
use super::bound::Bound;
use super::probe::{probe, Probe};
use crate::run::BindFn;
use crate::say::say;
use crate::station::{draw, Station};
use crate::xhci::{connected_ports, lookup};

pub struct Scanner {
    xhci: Option<u32>,
    /// Drawn once a run and handed to every bind: net.core asks for the
    /// address only when its bound port changes, so a rebind keeps it.
    station: Option<Station>,
    book: Book,
    backoff: Backoff,
}

impl Scanner {
    pub(crate) fn new(tag: &[u8]) -> Self {
        let station = draw();
        if station.is_none() {
            say(tag, &[b"no entropy for a station address, no device bound until there is"]);
        }
        Self { xhci: None, station, book: Book::default(), backoff: Backoff::new() }
    }

    /// The device, when a port this pass holds one this driver binds.
    pub fn pass<N>(&mut self, tag: &[u8], bind: BindFn<N>) -> Option<Bound<N>> {
        // No port is addressed before there is an address to hand its bind.
        self.station = self.station.or_else(draw);
        self.xhci = self.xhci.or_else(lookup);
        let (station, xhci) = (self.station?, self.xhci?);
        if self.backoff.due() {
            self.book.retry_all();
        }
        let ports = connected_ports(xhci).ok()?;
        for port in self.book.plan(&ports) {
            match probe(xhci, port, tag, bind, station) {
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

    /// A bound device stopped answering: its port counts a failure, so one
    /// that keeps failing is left after `TRIES`.
    pub fn lost(&mut self, port: u8) {
        self.book.failed(port);
    }
}
