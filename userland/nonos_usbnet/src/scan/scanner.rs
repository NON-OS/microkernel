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

//! The search for the device, one pass over every host's root ports at a time.

use nonos_libc::Deadline;

use super::book::Book;
use super::bound::Bound;
use super::probe::{probe, Probe};
use super::search::{search, Tried};
use crate::hosts::Hosts;
use crate::run::BindFn;
use crate::xhci::{connected_ports, host, hosts};

/// How often every port decided against is tried again, for a phone whose
/// switch to tethering the connect-change bit did not show.
const RETRY_ALL_MS: u64 = 60_000;

pub struct Scanner {
    hosts: Hosts<Book>,
    retry_all: Deadline,
}

impl Scanner {
    // A Default impl would change every network driver's ELF, which are
    // signed; the scanner is only ever made this way.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let retry_all = Deadline::after_ms(RETRY_ALL_MS);
        Self { hosts: hosts(Book::default), retry_all }
    }

    /// The device, when a port of a host this pass holds one this driver
    /// binds. A host that has registered since the last pass is searched.
    pub fn pass<N>(&mut self, tag: &[u8], bind: BindFn<N>) -> Option<Bound<N>> {
        if self.hosts.find(host) == 0 {
            return None;
        }
        if self.retry_all.expired() {
            self.hosts.each(|_, book| book.retry_all());
            self.retry_all = Deadline::after_ms(RETRY_ALL_MS);
        }
        let ports = |xhci| connected_ports(xhci).ok();
        search(&mut self.hosts, ports, |xhci, port| match probe(xhci, port, tag, bind) {
            Probe::Ours(bound) => Tried::Ours(bound),
            Probe::NotOurs => Tried::NotOurs,
            Probe::Busy => Tried::Busy,
            Probe::Failed => Tried::Failed,
        })
    }

    /// A bound device stopped answering: its port on its host counts a
    /// failure, so one that keeps failing is left after `TRIES`.
    pub fn lost(&mut self, xhci: u32, port: u8) {
        self.hosts.each(|host, book| {
            if host == xhci {
                book.failed(port);
            }
        });
    }
}
