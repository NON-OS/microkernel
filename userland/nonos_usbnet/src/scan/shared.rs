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
//! The search for devices another class driver configured and shares (a
//! composite key's FIDO interface beside its keyboard). A driver that
//! drives such interfaces keeps one of these beside its `Scanner`; the
//! ports it tries are the claimed ones, each once until it changes.

use alloc::vec::Vec;

use super::book::Book;
use super::bound::Bound;
use super::join::{join, ShareFn};
use super::port::Port;
use super::probe::Probe;
use super::search::{search, Tried};
use crate::hosts::Hosts;
use crate::xhci::{connected_ports, host, hosts, PORT_CLAIMED, PORT_FREE};

pub struct SharedScanner {
    hosts: Hosts<Book>,
}

impl Default for SharedScanner {
    fn default() -> Self {
        Self { hosts: hosts(Book::default) }
    }
}

impl SharedScanner {
    pub fn pass<N>(&mut self, tag: &[u8], share: ShareFn<N>) -> Option<Bound<N>> {
        if self.hosts.find(host) == 0 {
            return None;
        }
        // The book tries free ports; here the claimed ones stand for them.
        let claimed =
            |p: Port| Port { owner: if p.owner == PORT_CLAIMED { PORT_FREE } else { 1 }, ..p };
        let ports = |xhci| {
            connected_ports(xhci).ok().map(|ps| ps.into_iter().map(claimed).collect::<Vec<_>>())
        };
        search(&mut self.hosts, ports, |xhci, port| match join(xhci, port, tag, share) {
            Probe::Ours(bound) => Tried::Ours(bound),
            Probe::NotOurs => Tried::NotOurs,
            Probe::Busy => Tried::Busy,
            Probe::Failed => Tried::Failed,
        })
    }

    /// A shared device left: its port is tried afresh when one returns.
    pub fn lost(&mut self, xhci: u32, port: u8) {
        self.hosts.each(|host, book| {
            if host == xhci {
                book.failed(port);
            }
        });
    }
}
